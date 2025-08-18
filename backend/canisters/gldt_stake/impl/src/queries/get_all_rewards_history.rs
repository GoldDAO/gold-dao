use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_all_rewards_history::{
    Args as GetAllRewardsHistoryArgs, Response as GetAllRewardsHistoryResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
fn get_all_rewards_history() -> GetAllRewardsHistoryResponse {
    get_all_rewards_history_impl()
}

fn get_all_rewards_history_impl() -> GetAllRewardsHistoryResponse {
    read_state(|s| {
        s.data
            .allocated_rewards_pool
            .daily_allocated_rewards
            .clone()
    })
}
