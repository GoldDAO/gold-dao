use crate::jobs::process_rewards::batch_rewards_transfer;
use crate::jobs::process_rewards::process_rewards_impl;
use crate::model::processing_rewards_pool::ProcessingRewards;
use crate::{
    queries::{
        calculate_apy, calculate_daily_reward_per_token_in_usd, calculate_weighted_stake_usd,
        sum_usd_rewards,
    },
    state::{mutate_state, read_state},
};
use candid::Nat;
use canister_time::{run_now_then_interval, timestamp_millis, DAY_IN_MS, HOUR_IN_MS};
use futures::future::join_all;
use gldt_stake_api_canister::TimestampMillis;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::time::Duration;
use tracing::{error, info};
const MAX_RETRY_ATTEMPTS: u8 = 3;

pub fn start_job() {
    info!("START_JOB :: registering hourly run interval");
    run_now_then_interval(
        Duration::from_millis(HOUR_IN_MS),
        spawn_reward_allocation_job,
    );
}

fn spawn_reward_allocation_job() {
    info!("SPAWN_REWARD_ALLOCATION_JOB :: spawning async reward allocation job");
    ic_cdk::futures::spawn(handle_process_and_allocation())
}

async fn handle_process_and_allocation() {
    info!("HANDLE_PROCESS_AND_ALLOCATION :: start");

    let now = timestamp_millis();
    info!(
        "HANDLE_PROCESS_AND_ALLOCATION :: current timestamp = {}",
        now
    );

    if !is_allowed_to_run(now) {
        info!("HANDLE_PROCESS_AND_ALLOCATION :: not allowed to run at this time");
        return;
    }

    info!("HANDLE_PROCESS_AND_ALLOCATION :: running process_rewards_impl");
    match process_rewards_impl().await {
        Ok(_) => {
            info!("HANDLE_PROCESS_AND_ALLOCATION :: process_rewards_impl succeeded for all tokens");
            process_rewards_allocation_job_impl().await;
        }
        Err(failed_tokens) => {
            info!(
                "HANDLE_PROCESS_AND_ALLOCATION :: process_rewards_impl failed for {} tokens: {:?}",
                failed_tokens.len(),
                failed_tokens
            );
            let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());
            info!(
                "HANDLE_PROCESS_AND_ALLOCATION :: total reward types = {}",
                reward_types.len()
            );

            if failed_tokens.len() < reward_types.len() {
                info!(
                    "HANDLE_PROCESS_AND_ALLOCATION :: some tokens succeeded ({} of {}), running allocation before retry",
                    reward_types.len() - failed_tokens.len(),
                    reward_types.len()
                );
                process_rewards_allocation_job_impl().await;
            }

            handle_process_rewards_retry(
                failed_tokens.into_iter().map(|token| (token, 0)).collect(),
            )
            .await
        }
    }
}

pub async fn handle_process_rewards_retry(retry_counts: BTreeMap<TokenSymbol, u8>) {
    info!(
        "PROCESS_REWARDS_RETRY :: starting retry attempt for {} tokens: {:?}",
        retry_counts.len(),
        retry_counts
    );

    let retry_tokens: BTreeSet<TokenSymbol> = retry_counts.keys().cloned().collect();
    info!("PROCESS_REWARDS_RETRY :: retry_tokens = {:?}", retry_tokens);

    let unallocated_rewards_pool = read_state(|s| s.data.unallocated_rewards_pool.clone());

    let results = batch_rewards_transfer(retry_tokens.clone(), unallocated_rewards_pool).await;
    info!(
        "PROCESS_REWARDS_RETRY :: batch_rewards_transfer results = {:?}",
        results
    );

    let mut still_failing = BTreeMap::new();
    let mut succeeded_any = false;

    for (token_symbol, result) in results.into_iter() {
        match result {
            Ok(_) => {
                info!(
                    "PROCESS_REWARDS_RETRY :: token {} retry succeeded",
                    token_symbol
                );
                succeeded_any = true;
            }
            Err(err) => {
                let count = retry_counts.get(&token_symbol).copied().unwrap_or(0) + 1;
                error!(
                    "PROCESS_REWARDS_RETRY :: retry failed for token: {}, attempt: {}, error: {:?}",
                    token_symbol, count, err
                );

                if count >= MAX_RETRY_ATTEMPTS {
                    error!(
                        "PROCESS_REWARDS_RETRY :: giving up on token {} after {} attempts",
                        token_symbol, count
                    );
                } else {
                    still_failing.insert(token_symbol, count);
                }
            }
        }
    }

    if succeeded_any {
        info!("PROCESS_REWARDS_RETRY :: at least one token succeeded, running allocation");
        process_rewards_allocation_job_impl().await;
    }

    if !still_failing.is_empty() {
        info!(
            "PROCESS_REWARDS_RETRY :: scheduling retry in 5 minutes for tokens: {:?}",
            still_failing
        );
        ic_cdk_timers::set_timer(Duration::from_secs(60 * 5), move || {
            ic_cdk::futures::spawn(handle_process_rewards_retry(still_failing));
        });
    } else {
        info!("PROCESS_REWARDS_RETRY :: all tokens succeeded or max retries reached");
    }
}

async fn process_rewards_allocation_job_impl() {
    info!("PROCESS_REWARD_ALLOCATION_JOB_IMPL :: start");

    let awaiting = read_state(|s| s.data.allocated_rewards_pool.is_awaiting());
    info!(
        "PROCESS_REWARD_ALLOCATION_JOB_IMPL :: is_awaiting = {}",
        awaiting
    );
    if !awaiting {
        info!("PROCESS_REWARD_ALLOCATION_JOB_IMPL :: already in progress, exiting early");
        return;
    }

    mutate_state(|s| {
        s.data.allocated_rewards_pool.transition_to_allocating();
    });

    if let Err(e) = allocate_rewards().await {
        error!(
            "PROCESS_REWARD_ALLOCATION_JOB_IMPL :: allocation failed: {}",
            e
        );
    }

    calculate_daily_variables();

    mutate_state(|s| s.data.allocated_rewards_pool.transition_to_awaiting());

    info!("PROCESS_REWARD_ALLOCATION_JOB_IMPL :: finish");
}

pub async fn allocate_rewards() -> Result<(), String> {
    info!("ALLOCATE_REWARDS :: start");

    let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());
    info!("ALLOCATE_REWARDS :: reward_types = {:?}", reward_types);

    let processing_rewards_pool = read_state(|s| s.data.processing_rewards_pool.clone());
    let mut transfer_futures = Vec::new();
    let mut successful_reward_types = Vec::new();

    for reward_type in &reward_types {
        let token_info = reward_type.get_token_info();
        let balance = processing_rewards_pool
            .balance(token_info.ledger_id)
            .await?;
        info!(
            "ALLOCATE_REWARDS :: checking token {:?}, ledger_id = {}, fee = {}, balance = {}",
            reward_type,
            token_info.ledger_id.to_text(),
            token_info.fee,
            balance
        );

        if token_info.fee > balance {
            info!(
                "ALLOCATE_REWARDS :: insufficient balance for {:?}, skipping",
                reward_type
            );
            continue;
        }

        let rewards_to_allocate = balance.clone() - token_info.fee;
        info!(
            "ALLOCATE_REWARDS :: transferring rewards for {:?}, amount = {}",
            reward_type, rewards_to_allocate
        );

        let future =
            processing_rewards_pool.transfer_rewards(token_info.ledger_id, rewards_to_allocate);
        successful_reward_types.push(*reward_type);
        transfer_futures.push(future);
    }

    let results = join_all(transfer_futures).await;
    info!("ALLOCATE_REWARDS :: transfer results = {:?}", results);

    for (reward_type, transfer_result) in
        successful_reward_types.into_iter().zip(results.into_iter())
    {
        match transfer_result {
            Ok(rewards_to_allocate) => {
                info!(
                    "ALLOCATE_REWARDS :: successfully transferred for {:?}, amount = {}",
                    reward_type, rewards_to_allocate
                );
                allocate_rewards_change_state(reward_type, rewards_to_allocate.clone());
            }
            Err(err) => {
                error!(
                    "ALLOCATE_REWARDS :: transfer failed for {:?}, error = {}",
                    reward_type, err
                );
            }
        }
    }

    info!("ALLOCATE_REWARDS :: finish");
    Ok(())
}

fn calculate_daily_variables() {
    info!("CALCULATE_DAILY_VARIABLES :: start");
    let daily_apy_timestamp = read_state(|s| s.data.stake_system.daily_apy_timestamp);
    let now = timestamp_millis();
    let threshold = daily_apy_timestamp + DAY_IN_MS;

    info!(
        "CALCULATE_DAILY_VARIABLES :: now = {}, last daily APY ts = {}, threshold = {}",
        now, daily_apy_timestamp, threshold
    );

    if now < threshold {
        info!("CALCULATE_DAILY_VARIABLES :: skipping, not enough time passed");
        return;
    }

    let apy = calculate_daily_apy();
    let total_weighted_stake =
        read_state(|s| s.data.stake_system.cached_total_weighted_stake.clone());

    info!(
        "CALCULATE_DAILY_VARIABLES :: calculated APY = {}, total weighted stake = {:?}",
        apy, total_weighted_stake
    );

    mutate_state(|s| {
        let ts = timestamp_millis();
        s.data.stake_system.daily_apy_history.insert(ts, apy);
        s.data.stake_system.bump_daily_timestamp();
        s.data
            .stake_system
            .daily_weighted_staked_gldt
            .insert(ts, total_weighted_stake);
    });

    info!("CALCULATE_DAILY_VARIABLES :: finish");
}

fn calculate_daily_apy() -> f64 {
    info!("CALCULATE_DAILY_APY :: start");

    let (total_weighted_stake, daily_token_rewards, token_usd_values) = read_state(|s| {
        let stake_system = &s.data.stake_system;
        let rewards_pool = &s.data.allocated_rewards_pool;
        (
            stake_system.cached_total_weighted_stake.clone(),
            rewards_pool.daily_allocated_rewards.clone(),
            stake_system.token_usd_values.clone(),
        )
    });

    info!(
        "CALCULATE_DAILY_APY :: total_weighted_stake = {:?}, daily_token_rewards size = {}, token_usd_values = {:?}",
        total_weighted_stake, daily_token_rewards.len(), token_usd_values
    );

    if let Some((_, latest_daily_rewards)) = daily_token_rewards.iter().last() {
        info!(
            "CALCULATE_DAILY_APY :: latest daily rewards = {:?}",
            latest_daily_rewards
        );
        let daily_reward_per_token_usd = calculate_daily_reward_per_token_in_usd(
            latest_daily_rewards.clone(),
            1,
            &token_usd_values,
        );
        let total_rewards_usd = sum_usd_rewards(daily_reward_per_token_usd.clone());
        let weighted_stake_usd =
            calculate_weighted_stake_usd(total_weighted_stake.clone(), &token_usd_values);

        info!(
            "CALCULATE_DAILY_APY :: total_rewards_usd = {}, weighted_stake_usd = {}",
            total_rewards_usd, weighted_stake_usd
        );
        calculate_apy(total_rewards_usd, weighted_stake_usd)
    } else {
        info!("CALCULATE_DAILY_APY :: no rewards history found");
        0.0
    }
}

use crate::model::allocated_rewards_pool::calculate_total_weighted_stake;
use types::TokenSymbol;
pub fn allocate_rewards_change_state(reward_type: TokenSymbol, rewards_to_allocate: Nat) {
    info!(
        "ALLOCATE_REWARDS_CHANGE_STATE :: reward_type = {:?}, rewards_to_allocate = {}",
        reward_type, rewards_to_allocate
    );
    let now = timestamp_millis();
    let mut stake_positions =
        read_state(|s| s.data.stake_system.get_reward_eligible_stake_positions());
    let total_weighted_stake = calculate_total_weighted_stake(&stake_positions);
    let daily_apy_timestamp = read_state(|s| s.data.stake_system.daily_apy_timestamp);

    info!(
        "ALLOCATE_REWARDS_CHANGE_STATE :: total stake positions = {}, total_weighted_stake = {:?}",
        stake_positions.len(),
        total_weighted_stake
    );

    for (principal, position) in stake_positions.iter_mut() {
        let reward =
            position.calculate_new_reward(&total_weighted_stake, now, &rewards_to_allocate);
        info!(
            "ALLOCATE_REWARDS_CHANGE_STATE :: principal = {:?}, calculated reward = {}",
            principal, reward
        );

        position
            .claimable_rewards
            .entry(reward_type)
            .and_modify(|v| *v += reward.clone())
            .or_insert(reward.clone());

        mutate_state(|s| {
            s.data
                .stake_system
                .upsert_stake_position(*principal, position.clone());
        });
    }

    mutate_state(|s| {
        s.data
            .allocated_rewards_pool
            .add_to_reward_history(&reward_type, rewards_to_allocate.clone());
        s.data.allocated_rewards_pool.add_reward(
            daily_apy_timestamp,
            reward_type,
            rewards_to_allocate,
        );
        s.data.stake_system.cached_total_weighted_stake = total_weighted_stake.clone();
    });

    info!("ALLOCATE_REWARDS_CHANGE_STATE :: finish");
}

fn is_allowed_to_run(initial_run_time: TimestampMillis) -> bool {
    info!(
        "IS_ALLOWED_TO_RUN :: initial_run_time = {}",
        initial_run_time
    );

    let is_awaiting = read_state(|s| s.data.processing_rewards_pool.is_awaiting());
    let allocate_rewards_interval = match read_state(|s| s.data.allocate_rewards_interval.clone()) {
        Some(interval) => interval,
        None => {
            info!("IS_ALLOWED_TO_RUN :: no interval set, aborting");
            return false;
        }
    };

    let is_time_valid = allocate_rewards_interval.is_within_daily_interval(initial_run_time);
    info!(
        "IS_ALLOWED_TO_RUN :: is_awaiting = {}, is_time_valid = {}",
        is_awaiting, is_time_valid
    );

    if !is_awaiting {
        info!("IS_ALLOWED_TO_RUN :: allocation already in progress");
        return false;
    }
    is_time_valid
}
