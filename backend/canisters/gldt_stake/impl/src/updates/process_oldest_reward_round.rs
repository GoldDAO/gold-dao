use crate::guards::caller_is_governance_principal;
use crate::jobs::process_reward_rounds::allocate_rewards;
use crate::state::mutate_state;
use crate::state::read_state;
use crate::utils::commit_changes;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::process_oldest_reward_round::{
    Args as ProcessOldestRoundArgs, Response as ProcessOldestRoundResponse,
};
use gldt_stake_common::reward_round::RewardRoundStatus;
use ic_cdk::query;
use ic_cdk::update;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn process_oldest_reward_round_validate(
    _args: ProcessOldestRoundArgs,
) -> Result<String, String> {
    Ok("".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn process_oldest_reward_round(_args: ProcessOldestRoundArgs) -> ProcessOldestRoundResponse {
    process_oldest_reward_round_impl().await
}

async fn process_oldest_reward_round_impl() -> ProcessOldestRoundResponse {
    let latest_round = read_state(|s| s.data.reward_system.peek_oldest_round().cloned())
        .ok_or(format!("No rounds to process"))?;

    if read_state(|s| s.data.is_reward_allocation_in_progress) {
        return Err(format!(
            "process_reward_rounds cron job is already in progress"
        ));
    }
    mutate_state(|s| s.data.is_reward_allocation_in_progress = true);

    commit_changes().await;

    mutate_state(|s| {
        s.data
            .reward_system
            .set_oldest_round_status(RewardRoundStatus::AllocationInProgress)
    });

    commit_changes().await;

    allocate_rewards(latest_round);
    mutate_state(|s| s.data.is_reward_allocation_in_progress = false);
    return Ok(format!("rewards processed successfully"));
}
