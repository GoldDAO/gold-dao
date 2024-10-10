use ic_cdk_macros::query;

use crate::state::read_state;
pub use buyback_burn_api::get_swap::Args as GetSwapArgs;
pub use buyback_burn_api::get_swap::Response as GetSwapResponse;

#[query]
fn get_swap(args: GetSwapArgs) -> GetSwapResponse {
    read_state(|state| state.data.token_swaps.get_swap_info(args))
}
