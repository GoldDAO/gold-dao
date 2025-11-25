pub use gldt_swap_api_canister::get_active_swaps_by_ids::{
    Args as GetActiveSwapsByIdsArgs, Response as GetActiveSwapsByIdsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
pub fn get_active_swaps_by_ids(args: GetActiveSwapsByIdsArgs) -> GetActiveSwapsByIdsResponse {
    read_state(|s| s.data.swap_system.get_active_swap_by_ids(&args))
}
