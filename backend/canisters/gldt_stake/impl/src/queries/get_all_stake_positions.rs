use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_all_stake_positions::{
    Args as GetAllStakePositionsArgs, Response as GetAllStakePositionsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
fn get_all_stake_positions() -> GetAllStakePositionsResponse {
    get_all_stake_positions_impl()
}

fn get_all_stake_positions_impl() -> GetAllStakePositionsResponse {
    read_state(|s| s.data.stake_system.stakes.clone())
}
