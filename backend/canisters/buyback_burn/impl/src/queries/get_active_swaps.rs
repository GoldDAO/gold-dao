use ic_cdk_macros::query;
pub use buyback_burn_canister::get_active_swaps::Args as GetActiveSwapsArgs;
pub use buyback_burn_canister::get_active_swaps::Response as GetActiveSwapsResponse;
use crate::state::read_state;

#[query]
fn get_active_swaps(args: GetActiveSwapsArgs) -> GetActiveSwapsResponse {
    read_state(|state| state.data.token_swaps.get_active_swaps())
}
