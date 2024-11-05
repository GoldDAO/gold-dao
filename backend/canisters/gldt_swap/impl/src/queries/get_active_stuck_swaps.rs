pub use gldt_swap_api_canister::get_active_stuck_swaps::{
    Args as GetActiveStuckSwapsArgs, Response as GetActiveStuckSwapsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query(hidden = true)]
async fn get_active_stuck_swaps(_: GetActiveStuckSwapsArgs) -> GetActiveStuckSwapsResponse {
    read_state(|s| s.data.swaps.get_stuck_swaps())
}
