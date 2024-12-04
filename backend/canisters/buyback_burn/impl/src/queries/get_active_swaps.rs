use crate::state::read_state;
pub use buyback_burn_api::get_active_swaps::Response as GetActiveSwapsResponse;
use ic_cdk_macros::query;

#[query]
fn get_active_swaps() -> GetActiveSwapsResponse {
    read_state(|state| state.data.token_swaps.get_active_swaps())
}
