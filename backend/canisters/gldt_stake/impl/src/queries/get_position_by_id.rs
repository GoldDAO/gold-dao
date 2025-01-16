use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_position_by_id::{
    Args as GetPositionByIdArgs, Response as GetPositionByIdResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
fn get_position_by_id(position_id: GetPositionByIdArgs) -> GetPositionByIdResponse {
    get_position_by_id_impl(position_id)
}

fn get_position_by_id_impl(position_id: GetPositionByIdArgs) -> GetPositionByIdResponse {
    // 1. check user isn't anon
    let now = timestamp_millis();
    read_state(|s| s.data.stake_system.get_stake_position(position_id))
        .map(|p| (p, now, position_id).into())
}
