use bity_ic_canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_total_allocated_rewards::{
    Args as GetTotalAllocatedRewardsArgs, Response as GetTotalAllocatedRewardsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
fn get_total_allocated_rewards(
    args: GetTotalAllocatedRewardsArgs,
) -> GetTotalAllocatedRewardsResponse {
    get_total_allocated_rewards_impl(args)
}

fn get_total_allocated_rewards_impl(
    _: GetTotalAllocatedRewardsArgs,
) -> GetTotalAllocatedRewardsResponse {
    read_state(|s| s.data.allocated_rewards_pool.reward_history.clone())
}
