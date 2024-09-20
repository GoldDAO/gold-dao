pub use gldt_swap_api_canister::get_active_swaps::{
    Args as GetActiveSwapsArgs,
    Response as GetActiveSwapsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
async fn get_active_swaps(_: GetActiveSwapsArgs) -> GetActiveSwapsResponse {
    read_state(|s| s.data.swaps.get_active_swaps())
}
