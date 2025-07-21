use crate::model::processing_rewards_pool::ProcessingRewards;
use crate::{
    queries::{
        calculate_apy, calculate_daily_reward_per_token_in_usd, calculate_weighted_stake_usd,
        sum_usd_rewards,
    },
    state::{mutate_state, read_state},
};
use candid::Nat;
use canister_time::{run_interval, timestamp_millis, DAY_IN_MS, HOUR_IN_MS};
use futures::future::join_all;
use gldt_stake_api_canister::TimestampMillis;
use std::time::Duration;
use tracing::error;
use tracing::info;

pub fn start_job() {
    run_interval(
        Duration::from_millis(HOUR_IN_MS),
        spawn_reward_allocation_job,
    );
}

fn spawn_reward_allocation_job() {
    ic_cdk::futures::spawn(process_rewards_allocation_job_impl())
}

async fn process_rewards_allocation_job_impl() {
    info!("PROCESS REWARD ALLOCATION :: start");

    let now = timestamp_millis();
    if !is_allowed_to_run(now) {
        return;
    }

    if read_state(|s| !s.data.allocated_rewards_pool.is_awaiting()) {
        info!("PROCESS REWARD ALLOCATION :: already in progress, exiting early");
        return;
    }

    mutate_state(|s| s.data.allocated_rewards_pool.transition_to_allocating());

    let _ = allocate_rewards().await;

    calculate_daily_variables();

    mutate_state(|s| s.data.allocated_rewards_pool.transition_to_awaiting());

    info!("PROCESS REWARD ALLOCATION :: finish");
}

pub async fn allocate_rewards() -> Result<(), String> {
    info!("ALLOCATE REWARDS :: attempting to allocate");

    let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());
    let processing_rewards_pool = read_state(|s| s.data.processing_rewards_pool.clone());
    let mut transfer_futures = Vec::new();

    for reward_type in &reward_types {
        let token_ledger_id = reward_type.get_token_info().ledger_id;
        let token_fee = reward_type.get_token_info().fee;

        let balance = processing_rewards_pool.balance(token_ledger_id).await?;

        if token_fee > balance {
            let error_message = format!(
            "ALLOCATE REWARDS :: error - insufficient balance for token: {:?}, ledger_id: {}, balance: {}, fee: {}",
            reward_type,
            token_ledger_id.to_text(),
            balance,
            token_fee
        );
            info!(error_message);
            continue;
        }

        let rewards_to_allocate = balance.clone() - token_fee;

        let future =
            processing_rewards_pool.transfer_rewards(token_ledger_id, rewards_to_allocate.clone());

        transfer_futures.push(future);
    }

    let results = join_all(transfer_futures).await;

    for (reward_type, transfer_result) in reward_types.into_iter().zip(results.into_iter()) {
        match transfer_result {
            Ok(rewards_to_allocate) => {
                info!(
                    "ALLOCATE REWARDS :: committed to state for {}, amount: {}",
                    reward_type, rewards_to_allocate
                );

                allocate_rewards_change_state(reward_type, rewards_to_allocate.clone())
            }
            Err(err) => {
                error!(
                    "ALLOCATE REWARDS :: transfer failed for token: {}, error: {}",
                    reward_type, err
                );
            }
        }
    }

    info!("ALLOCATE REWARDS :: all transfers processed");

    Ok(())
}

fn calculate_daily_variables() {
    let daily_apy_timestamp = read_state(|s| s.data.stake_system.daily_apy_timestamp);

    let now = timestamp_millis();
    let threshold = daily_apy_timestamp + DAY_IN_MS;

    if now < threshold {
        info!("CALCULATE DAILY VARIABLES :: skipping, not enough time passed");
        return;
    }

    let apy = calculate_daily_apy();
    let total_weighted_stake =
        read_state(|s| s.data.stake_system.cached_total_weighted_stake.clone());

    mutate_state(|s| {
        s.data
            .stake_system
            .daily_apy_history
            .insert(timestamp_millis(), apy);
        s.data.stake_system.bump_daily_timestamp();
        s.data
            .stake_system
            .daily_weighted_staked_gldt
            .insert(timestamp_millis(), total_weighted_stake);
    });
}

fn calculate_daily_apy() -> f64 {
    info!("CALCULATE DAILY APY :: start");

    let (total_weighted_stake, daily_token_rewards, token_usd_values) = read_state(|s| {
        let stake_system = &s.data.stake_system;
        let rewards_pool = &s.data.allocated_rewards_pool;

        (
            stake_system.cached_total_weighted_stake.clone(),
            rewards_pool.daily_allocated_rewards.clone(),
            stake_system.token_usd_values.clone(),
        )
    });

    if let Some((_, latest_daily_rewards)) = daily_token_rewards.iter().last() {
        let daily_reward_per_token_usd = calculate_daily_reward_per_token_in_usd(
            latest_daily_rewards.clone(),
            1,
            &token_usd_values,
        );

        let total_rewards_usd = sum_usd_rewards(daily_reward_per_token_usd);

        let weighted_stake_usd =
            calculate_weighted_stake_usd(total_weighted_stake.clone(), &token_usd_values);
        calculate_apy(total_rewards_usd, weighted_stake_usd)
    } else {
        0.0
    }
}

use crate::model::allocated_rewards_pool::calculate_total_weighted_stake;
use types::TokenSymbol;
pub fn allocate_rewards_change_state(reward_type: TokenSymbol, rewards_to_allocate: Nat) {
    let now = timestamp_millis();

    let mut stake_positions =
        read_state(|s| s.data.stake_system.get_reward_eligible_stake_positions());
    let total_weighted_stake = calculate_total_weighted_stake(&stake_positions);
    let daily_apy_timestamp = read_state(|s| s.data.stake_system.daily_apy_timestamp);

    // Distribute to users
    for (principal, position) in stake_positions.iter_mut() {
        let reward =
            position.calculate_new_reward(&total_weighted_stake, now, &rewards_to_allocate);

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

    // Update cached state after all rewards are allocated
    mutate_state(|s| {
        // Update reward history
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
}

fn is_allowed_to_run(initial_run_time: TimestampMillis) -> bool {
    let is_awaiting = read_state(|s| s.data.processing_rewards_pool.is_awaiting());
    let distribution_interval = match read_state(|s| s.data.reward_claim_interval.clone()) {
        Some(interval) => interval,
        None => {
            info!("PROCESS REWARD ALLOCATION :: no interval set, aborting");
            return false;
        }
    };

    let is_distribution_time_valid =
        distribution_interval.is_within_daily_interval(initial_run_time);

    if !is_awaiting {
        info!("PROCESS REWARD ALLOCATION :: allocation already in progress");
        return false;
    }
    if is_distribution_time_valid {
        return true;
    }
    false
}
