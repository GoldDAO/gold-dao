pub use gldt_swap_api_canister::get_failed_swaps::{
    Args as GetFailedSwapsArgs, Response as GetFailedSwapsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
async fn get_failed_swaps(_: GetFailedSwapsArgs) -> GetFailedSwapsResponse {
    read_state(|s| s.data.swap_system.get_failed_swaps())
}
