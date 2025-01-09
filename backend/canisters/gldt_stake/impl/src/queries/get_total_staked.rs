use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_total_staked::{
    Args as GetTotalStakedArgs, Response as GetTotalStakedResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
fn get_total_staked(_args: GetTotalStakedArgs) -> GetTotalStakedResponse {
    get_total_staked_impl(_args)
}

fn get_total_staked_impl(_args: GetTotalStakedArgs) -> GetTotalStakedResponse {
    read_state(|s| s.data.stake_system.total_staked.clone())
}
