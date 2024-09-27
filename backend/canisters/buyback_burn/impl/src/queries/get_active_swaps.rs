use ic_cdk_macros::query;
pub use buyback_burn_api::get_active_swaps::Args as GetActiveSwapsArgs;
pub use buyback_burn_api::get_active_swaps::Response as GetActiveSwapsResponse;
use crate::state::read_state;

#[query]
fn get_active_swaps() -> GetActiveSwapsResponse {
    read_state(|state| state.data.token_swaps.get_active_swaps())
}
