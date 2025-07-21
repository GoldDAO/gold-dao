use canister_time::timestamp_millis;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_position::{
    Args as GetPositionByIdArgs, Response as GetPositionByIdResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
fn get_position() -> GetPositionByIdResponse {
    get_position_impl()
}

fn get_position_impl() -> GetPositionByIdResponse {
    let now = timestamp_millis();
    let caller = ic_cdk::api::msg_caller();
    read_state(|s| s.data.stake_system.get_stake_position(&caller)).map(|p| (p, now).into())
}
