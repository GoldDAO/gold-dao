pub use gldt_swap_api_canister::get_failed_swaps_by_ids::{
    Args as GetFailedSwapsByIdsArgs, Response as GetFailedSwapsByIdsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
pub fn get_failed_swaps_by_ids(args: GetFailedSwapsByIdsArgs) -> GetFailedSwapsByIdsResponse {
    read_state(|s| s.data.swap_system.get_failed_swap_by_ids(&args))
}
