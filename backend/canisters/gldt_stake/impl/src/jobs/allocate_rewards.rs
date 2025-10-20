use crate::jobs::process_rewards::{
    batch_rewards_transfer_unlimited, batch_rewards_transfer_with_limit, process_rewards_impl,
};
use crate::model::allocated_rewards_pool::calculate_total_weighted_stake;
use crate::model::processing_rewards_pool::ProcessingRewards;
use crate::state::{mutate_state, read_state};
use candid::Nat;
use candid::Principal;
use canister_time::{run_now_then_interval, timestamp_millis, HOUR_IN_MS};
use futures::future::join_all;
use gldt_stake_api_canister::TimestampMillis;
use gldt_stake_common::stake_position::StakePosition;
use std::collections::HashMap;
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
    ic_cdk::futures::spawn(handle_process_and_allocation_impl())
}

async fn handle_process_and_allocation_impl() {
    let now = timestamp_millis();

    if !is_allowed_to_run(now) {
        return;
    }

    handle_process_and_allocation().await
}

pub async fn handle_process_and_allocation() {
    let _span = tracing::info_span!("HANDLE_PROCESS_AND_ALLOCATION").entered();

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

    info!("finish");
}

pub async fn handle_process_rewards_retry(retry_counts: BTreeMap<TokenSymbol, u8>) {
    let _span = tracing::info_span!("PROCESS_REWARDS_RETRY").entered();

    let retry_tokens: BTreeSet<TokenSymbol> = retry_counts.keys().cloned().collect();
    let unallocated_rewards_pool = read_state(|s| s.data.unallocated_rewards_pool.clone());

    let apy_limit_opt = read_state(|s| s.data.stake_system.apy_limit);
    let results = if let Some(apy_limit) = apy_limit_opt {
        batch_rewards_transfer_with_limit(apy_limit, retry_tokens, unallocated_rewards_pool).await
    } else {
        batch_rewards_transfer_unlimited(retry_tokens, unallocated_rewards_pool).await
    };

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
    let mut allocated_rewards = HashMap::new();

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

                allocate_rewards_change_state(reward_type, rewards_to_allocate.clone());
                allocated_rewards.insert(reward_type, rewards_to_allocate);
            }
            Err(err) => {
                error!("transfer failed for {:?}, error = {}", reward_type, err);
            }
        }
    }

    ic_cdk::println!("Allocated rewards: {:?}", allocated_rewards);
    mutate_state(|s| {
        s.data.analytics_system.insert_daily_analytics(
            allocated_rewards,
            s.data.stake_system.total_staked.clone(),
            s.data.stake_system.cached_total_weighted_stake.clone(),
        );
    });

    mutate_state(|s| s.data.allocated_rewards_pool.transition_to_awaiting());
    info!("finish");
    Ok(())
}

pub fn allocate_rewards_change_state(reward_type: TokenSymbol, rewards_to_allocate: Nat) {
    let _span = tracing::info_span!("ALLOCATE_REWARDS_CHANGE_STATE").entered();
    info!("start");

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

    mutate_state(|s| {
        for (principal, position) in updated_positions {
            s.data
                .stake_system
                .upsert_stake_position(principal, position);
        }
        s.data
            .allocated_rewards_pool
            .add_to_reward_history(&reward_type, rewards_to_allocate.clone());

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
