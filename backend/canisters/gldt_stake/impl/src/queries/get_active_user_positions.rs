use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_active_user_positions::{
    Args as GetActiveUserPositionsArgs, Response as GetActiveUserPositionsResponse,
};
use ic_cdk::{caller, query};

use crate::state::read_state;

#[query]
#[trace]
fn get_active_user_positions(args: GetActiveUserPositionsArgs) -> GetActiveUserPositionsResponse {
    get_active_user_positions_impl(args)
}

fn get_active_user_positions_impl(
    args: GetActiveUserPositionsArgs,
) -> GetActiveUserPositionsResponse {
    // 1. check user isn't anon
    let user = args.unwrap_or(caller());
    let now = timestamp_millis();
    read_state(|s| s.data.stake_system.get_stake_positions_by_user(&user))
        .into_iter()
        .map(|(id, position)| (position, now, id).into())
        .collect()
}
