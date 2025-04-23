use crate::{
    queries::{
        calculate_apy, calculate_weekly_reward_per_token_in_usd, calculate_weighted_stake_usd,
        sum_usd_rewards,
    },
    state::{mutate_state, read_state},
};
use candid::Nat;
use canister_time::{run_interval, timestamp_millis, HOUR_IN_MS, WEEK_IN_MS};
use gldt_stake_common::reward_round::{RewardRound, RewardRoundStatus};
use std::time::Duration;
use tracing::info;

pub fn start_job() {
    run_interval(
        Duration::from_millis(HOUR_IN_MS),
        process_reward_rounds_job_impl,
    );
}

fn process_reward_rounds_job_impl() {
    info!("PROCESS REWARD ROUND :: start");
    if read_state(|s| s.data.is_reward_allocation_in_progress) {
        info!("PROCESS REWARD ROUND :: already in progress, exiting early");
        return;
    }
    mutate_state(|s| s.data.is_reward_allocation_in_progress = true);
    let rounds = read_state(|s| s.data.reward_system.get_all_reward_rounds());

    // oldest first
    for round in rounds {
        allocate_rewards(round);
    }
    calculate_weekly_variables();

    mutate_state(|s| s.data.is_reward_allocation_in_progress = false);
    info!("PROCESS REWARD ROUND :: finish");
}

pub fn allocate_rewards(round: RewardRound) {
    info!(
        "ALLOCATE REWARDS :: attempting to allocate {} {}",
        round.rewards, round.token_symbol
    );
    mutate_state(|s| {
        s.data
            .reward_system
            .set_oldest_round_status(RewardRoundStatus::AllocationInProgress)
    });
    let mut stake_positions =
        read_state(|s| s.data.stake_system.get_reward_eligible_stake_positions());
    let weekly_apy_timestamp = read_state(|s| s.data.stake_system.weekly_apy_timestamp);

    let total_weighted_stake = round.calculate_total_weighted_stake(&stake_positions);

    stake_positions.iter_mut().for_each(|(id, position)| {
        let rewards = round.get_rewards();
        let token_symbol = round.get_token_symbol();

        let reward = position.calculate_new_reward(
            &total_weighted_stake,
            round.get_round_timestamp(),
            rewards,
        );
        position
            .claimable_rewards
            .entry(token_symbol.clone())
            .and_modify(|value: &mut Nat| *value += reward.clone())
            .or_insert(reward);

        mutate_state(|s| {
            s.data
                .stake_system
                .update_stake_position(id, position.clone())
        });
    });

    mutate_state(|s| {
        s.data
            .reward_system
            .set_oldest_round_status(RewardRoundStatus::RewardsAllocated);
        s.data.stake_system.cached_total_weighted_stake = total_weighted_stake;
        s.data
            .reward_system
            .add_to_reward_history(round.get_token_symbol(), round.get_rewards().clone());
        s.data.reward_system.add_reward(
            weekly_apy_timestamp,
            round.get_token_symbol().clone(),
            round.get_rewards().clone(),
        );
        s.data.reward_system.remove_oldest_round()
    });
    info!(
        "ALLOCATE REWARDS :: allocated {} {} successfully",
        round.rewards, round.token_symbol
    );
}

fn calculate_weekly_variables() {
    let weekly_apy_timestamp = read_state(|s| s.data.stake_system.weekly_apy_timestamp);

    let now = timestamp_millis();
    let threshold = weekly_apy_timestamp + WEEK_IN_MS;

    if now < threshold {
        info!("CALCULATE WEEKLY VARIABLES :: more than one week must pass in order to calculate the weekly APY");
        return;
    }

    let apy = calculate_weekly_apy();
    let total_weighted_stake =
        read_state(|s| s.data.stake_system.cached_total_weighted_stake.clone());

    mutate_state(|s| {
        s.data
            .stake_system
            .weekly_apy_history
            .insert(timestamp_millis(), apy);
        s.data.stake_system.bump_weekly_timestamp();
        s.data
            .stake_system
            .weekly_weighted_staked_gldt
            .insert(timestamp_millis(), total_weighted_stake);
    });
}

fn calculate_weekly_apy() -> f64 {
    info!("CALCULATE WEEKLY APY :: start");
    let (total_weighted_stake, weekly_token_rewards, token_usd_values) = read_state(|s| {
        let stake_system = &s.data.stake_system;
        let reward_system = &s.data.reward_system;
        (
            stake_system.cached_total_weighted_stake.clone(),
            reward_system.weekly_allocated_rewards.clone(),
            stake_system.token_usd_values.clone(),
        )
    });
    if let Some((_, latest_weekly_rewards)) = weekly_token_rewards.iter().last() {
        let total_rewards_usd = sum_usd_rewards(calculate_weekly_reward_per_token_in_usd(
            latest_weekly_rewards.clone(),
            1,
            &token_usd_values,
        ));

        let weighted_stake_usd =
            calculate_weighted_stake_usd(total_weighted_stake, &token_usd_values);

        calculate_apy(total_rewards_usd, weighted_stake_usd)
    } else {
        0.0
    }
}
