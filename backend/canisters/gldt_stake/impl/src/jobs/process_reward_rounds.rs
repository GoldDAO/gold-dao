use crate::state::{mutate_state, read_state};
use candid::Nat;
use canister_time::{run_interval, HOUR_IN_MS};
use gldt_stake_common::reward_round::{RewardRound, RewardRoundStatus};
use std::time::Duration;
use tracing::info;

pub fn start_job() {
    run_interval(
        Duration::from_millis(HOUR_IN_MS),
        process_reward_rounds_job_impl,
    );
}

// todo
// when a round's rewards have been allocated. the total of that reward is added to a global amount
// we also keep track of when the canister went live and so we have a created date to figure out the number
// of weeks that have passed so that we can figure out the APY

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
    let now = round.get_round_timestamp();

    let total_weighted_stake =
        stake_positions
            .iter()
            .fold(Nat::from(0u64), |acc, (_, position)| {
                let age_bonus_multiplier = position.calculate_age_bonus_multiplier(now);
                let weighted_stake = position.calculate_weighted_stake(age_bonus_multiplier);
                acc + weighted_stake
            });

    stake_positions.iter_mut().for_each(|(id, position)| {
        let rewards = round.get_rewards();
        let token_symbol = round.get_token_symbol();

        let reward = position.calculate_new_reward(&total_weighted_stake, now, rewards);
        position
            .claimable_rewards
            .entry(token_symbol.clone())
            .and_modify(|value: &mut Nat| *value += reward.clone())
            .or_insert(reward);

        mutate_state(|s| {
            s.data
                .stake_system
                .update_stake_position(&id, position.clone())
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
        s.data.reward_system.remove_oldest_round()
    });
    info!(
        "ALLOCATE REWARDS :: allocated {} {} successfully",
        round.rewards, round.token_symbol
    );
}
