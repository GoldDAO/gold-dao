use bity_ic_canister_time::timestamp_millis;
use bity_ic_canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_position::{
    Args as GetPositionByIdArgs, Response as GetPositionByIdResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
fn get_position(args: GetPositionByIdArgs) -> GetPositionByIdResponse {
    get_position_impl(args)
}

fn get_position_impl(args: GetPositionByIdArgs) -> GetPositionByIdResponse {
    let now = timestamp_millis();
    read_state(|s| s.data.stake_system.get_stake_position(&args)).map(|p| (p, now).into())
}
