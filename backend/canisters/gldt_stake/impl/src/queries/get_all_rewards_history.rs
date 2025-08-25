use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_all_rewards_history::{
    Args as GetAllRewardsHistoryArgs, Response as GetAllRewardsHistoryResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
fn get_all_rewards_history(args: GetAllRewardsHistoryArgs) -> GetAllRewardsHistoryResponse {
    get_all_rewards_history_impl(args)
}

fn get_all_rewards_history_impl(args: GetAllRewardsHistoryArgs) -> GetAllRewardsHistoryResponse {
    read_state(|s| {
        s.data
            .analytics_system
            .get_rewards(args.starting_day, args.limit)
    })
}
