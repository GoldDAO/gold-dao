use crate::jobs::process_rewards::batch_rewards_transfer;
use crate::jobs::process_rewards::process_rewards_impl;
use crate::model::allocated_rewards_pool::calculate_total_weighted_stake;
use crate::model::processing_rewards_pool::ProcessingRewards;
use crate::{
    queries::{
        calculate_apy, calculate_daily_reward_per_token_in_usd, calculate_weighted_stake_usd,
        sum_usd_rewards,
    },
    state::{mutate_state, read_state},
};
use candid::Nat;
use candid::Principal;
use canister_time::{run_now_then_interval, timestamp_millis, DAY_IN_MS, HOUR_IN_MS};
use futures::future::join_all;
use gldt_stake_api_canister::TimestampMillis;
use gldt_stake_common::stake_position::StakePosition;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;
use tracing::{error, info};
use types::TokenSymbol;
const MAX_RETRY_ATTEMPTS: u8 = 3;

pub fn start_job() {
    run_now_then_interval(
        Duration::from_millis(HOUR_IN_MS),
        spawn_reward_allocation_job,
    );
}

fn spawn_reward_allocation_job() {
    ic_cdk::futures::spawn(handle_process_and_allocation())
}

async fn handle_process_and_allocation() {
    let _span = tracing::info_span!("HANDLE_PROCESS_AND_ALLOCATION").entered();

    let now = timestamp_millis();

    if !is_allowed_to_run(now) {
        return;
    }

    info!("start");

    match process_rewards_impl().await {
        Ok(_) => {
            info!("process_rewards_impl succeeded for all tokens");
            let _ = allocate_rewards().await;
        }
        Err(failed_tokens) => {
            error!(
                "process_rewards_impl failed for {} tokens: {:?}",
                failed_tokens.len(),
                failed_tokens
            );
            let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());

            if failed_tokens.len() < reward_types.len() {
                let _ = allocate_rewards().await;
            }

            handle_process_rewards_retry(
                failed_tokens.into_iter().map(|token| (token, 0)).collect(),
            )
            .await
        }
    }

    calculate_daily_variables();

    info!("finish");
}

pub async fn handle_process_rewards_retry(retry_counts: BTreeMap<TokenSymbol, u8>) {
    let _span = tracing::info_span!("PROCESS_REWARDS_RETRY").entered();

    let retry_tokens: BTreeSet<TokenSymbol> = retry_counts.keys().cloned().collect();
    let unallocated_rewards_pool = read_state(|s| s.data.unallocated_rewards_pool.clone());

    let results = batch_rewards_transfer(retry_tokens.clone(), unallocated_rewards_pool).await;

    let mut still_failing = BTreeMap::new();
    let mut succeeded = Vec::new();

    for (token_symbol, result) in results {
        match result {
            Ok(_) => {
                info!("token {} retry succeeded", token_symbol);
                succeeded.push(token_symbol);
            }
            Err(err) => {
                let count = retry_counts.get(&token_symbol).copied().unwrap_or(0) + 1;
                error!(
                    "retry failed for token: {}, attempt: {}, error: {:?}",
                    token_symbol, count, err
                );

                if count < MAX_RETRY_ATTEMPTS {
                    still_failing.insert(token_symbol, count);
                } else {
                    error!(
                        "giving up on token {} after {} attempts",
                        token_symbol, count
                    );
                }
            }
        }
    }

    if !succeeded.is_empty() {
        info!("at least one token succeeded, running allocation");
        let _ = allocate_rewards().await;
    }

    if !still_failing.is_empty() {
        info!(
            "scheduling retry in 5 minutes for tokens: {:?}",
            still_failing
        );
        ic_cdk_timers::set_timer(Duration::from_secs(60 * 5), move || {
            ic_cdk::futures::spawn(handle_process_rewards_retry(still_failing));
        });
    } else {
        info!("all tokens succeeded or max retries reached");
    }
}

pub async fn allocate_rewards() -> Result<(), String> {
    let _span = tracing::info_span!("ALLOCATE_REWARDS").entered();

    info!("start");

    mutate_state(|s| {
        s.data.allocated_rewards_pool.transition_to_allocating();
    });

    let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());

    let processing_rewards_pool = read_state(|s| s.data.processing_rewards_pool.clone());
    let mut transfer_futures = Vec::new();
    let mut reward_types_to_allocate = Vec::new();

    for reward_type in &reward_types {
        let token_info = reward_type.get_token_info();
        let balance = processing_rewards_pool
            .balance(token_info.ledger_id)
            .await?;

        if token_info.fee > balance {
            info!("insufficient balance = {} for {:?}", balance, reward_type);
            continue;
        }

        let rewards_to_allocate = balance - token_info.fee;

        transfer_futures.push(
            processing_rewards_pool.transfer_rewards(token_info.ledger_id, rewards_to_allocate),
        );
        reward_types_to_allocate.push(*reward_type);
    }

    let results = join_all(transfer_futures).await;

    for (reward_type, transfer_result) in reward_types_to_allocate
        .into_iter()
        .zip(results.into_iter())
    {
        match transfer_result {
            Ok(rewards_to_allocate) => {
                info!(
                    "successfully transferred for {:?}, amount = {}",
                    reward_type, rewards_to_allocate
                );
                ic_cdk::println!(
                    "successfully transferred for {:?}, amount = {}",
                    reward_type,
                    rewards_to_allocate
                );
                allocate_rewards_change_state(reward_type, rewards_to_allocate.clone());
            }
            Err(err) => {
                error!("transfer failed for {:?}, error = {}", reward_type, err);
            }
        }
    }

    mutate_state(|s| s.data.allocated_rewards_pool.transition_to_awaiting());
    info!("finish");
    Ok(())
}

fn calculate_daily_variables() {
    let _span = tracing::info_span!("CALCULATE_DAILY_VARIABLES").entered();

    info!("start");

    let daily_apy_timestamp = read_state(|s| s.data.stake_system.cached_daily_timestamp);
    let now = timestamp_millis();
    let threshold = daily_apy_timestamp + DAY_IN_MS;

    if now < threshold {
        info!("skipping, not enough time passed");
        return;
    }

    let apy = calculate_daily_apy();
    let total_weighted_stake =
        read_state(|s| s.data.stake_system.cached_total_weighted_stake.clone());

    info!(
        "calculated APY = {}, total weighted stake = {:?}",
        apy, total_weighted_stake
    );

    mutate_state(|s| {
        s.data.analytics_system.insert_daily_analytics(
            apy,
            s.data.stake_system.total_staked.clone(),
            total_weighted_stake,
            s.data.allocated_rewards_pool.cached_rewards.clone(),
        );

        s.data.allocated_rewards_pool.clear_cached_rewards();

        s.data.stake_system.bump_daily_timestamp();
    });

    info!("finish");
}

fn calculate_daily_apy() -> f64 {
    let _span = tracing::info_span!("CALCULATE_DAILY_APY").entered();

    info!("start");

    let (total_weighted_stake, cached_rewards, token_usd_values) = read_state(|s| {
        let stake_system = &s.data.stake_system;
        let rewards_pool = &s.data.allocated_rewards_pool;
        (
            stake_system.cached_total_weighted_stake.clone(),
            rewards_pool.cached_rewards.clone(),
            stake_system.token_usd_values.clone(),
        )
    });

    info!(
        "total_weighted_stake = {:?}, cached_rewards size = {}, token_usd_values = {:?}",
        total_weighted_stake,
        cached_rewards.len(),
        token_usd_values
    );

    if !cached_rewards.is_empty() {
        info!("cached rewards = {:?}", cached_rewards);

        let daily_reward_per_token_usd =
            calculate_daily_reward_per_token_in_usd(cached_rewards.clone(), 1, &token_usd_values);

        let total_rewards_usd = sum_usd_rewards(daily_reward_per_token_usd.clone());
        let weighted_stake_usd =
            calculate_weighted_stake_usd(total_weighted_stake.clone(), &token_usd_values);

        info!(
            "total_rewards_usd = {}, weighted_stake_usd = {}",
            total_rewards_usd, weighted_stake_usd
        );

        calculate_apy(total_rewards_usd, weighted_stake_usd)
    } else {
        info!("no cached rewards found for today");
        0.0
    }
}

pub fn allocate_rewards_change_state(reward_type: TokenSymbol, rewards_to_allocate: Nat) {
    let _span = tracing::info_span!("ALLOCATE_REWARDS_CHANGE_STATE").entered();
    info!(
        "reward_type = {:?}, rewards_to_allocate = {}",
        reward_type, rewards_to_allocate
    );

    let now = timestamp_millis();
    let mut stake_positions =
        read_state(|s| s.data.stake_system.get_reward_eligible_stake_positions());
    let total_weighted_stake = calculate_total_weighted_stake(&stake_positions);

    info!(
        "total stake positions = {}, total_weighted_stake = {:?}",
        stake_positions.len(),
        total_weighted_stake
    );

    // update all positions
    let updated_positions: Vec<(Principal, StakePosition)> = stake_positions
        .iter_mut()
        .map(|(principal, position)| {
            let reward =
                position.calculate_new_reward(&total_weighted_stake, now, &rewards_to_allocate);
            position
                .claimable_rewards
                .entry(reward_type)
                .and_modify(|v| *v += reward.clone())
                .or_insert(reward);
            (*principal, position.clone())
        })
        .collect();

    // apply state updates in a single mutate_state
    mutate_state(|s| {
        for (principal, position) in updated_positions {
            s.data
                .stake_system
                .upsert_stake_position(principal, position);
        }
        s.data
            .allocated_rewards_pool
            .add_to_reward_history(&reward_type, rewards_to_allocate.clone());
        s.data
            .allocated_rewards_pool
            .add_to_cached_rewards(reward_type, rewards_to_allocate);
        s.data.stake_system.cached_total_weighted_stake = total_weighted_stake.clone();
    });

    info!("finish");
}

fn is_allowed_to_run(initial_run_time: TimestampMillis) -> bool {
    let _span = tracing::info_span!("IS_ALLOWED_TO_RUN").entered();

    let is_awaiting = read_state(|s| {
        s.data.processing_rewards_pool.is_awaiting() && s.data.allocated_rewards_pool.is_awaiting()
    });
    let allocate_rewards_interval = match read_state(|s| s.data.allocate_rewards_interval.clone()) {
        Some(interval) => interval,
        None => {
            info!("no interval set, aborting");
            return false;
        }
    };

    let is_time_valid = allocate_rewards_interval.is_within_daily_interval(initial_run_time);

    if !is_awaiting {
        info!("allocation already in progress");
        return false;
    }
    is_time_valid
}
