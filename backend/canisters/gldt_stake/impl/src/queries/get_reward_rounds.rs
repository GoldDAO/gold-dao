use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_reward_rounds::{
    Args as GetRewardRoundsArgs, Response as GetRewardRoundsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
fn get_reward_rounds(args: GetRewardRoundsArgs) -> GetRewardRoundsResponse {
    get_reward_rounds_impl(args)
}

fn get_reward_rounds_impl(_: GetRewardRoundsArgs) -> GetRewardRoundsResponse {
    read_state(|s| s.data.reward_system.get_all_reward_rounds())
}
