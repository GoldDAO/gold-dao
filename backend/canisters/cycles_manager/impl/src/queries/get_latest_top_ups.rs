use crate::guards::caller_is_governance_principal;
use crate::state::read_state;
pub use cycles_manager_api_canister::get_latest_top_ups::Response as TopUpsResponse;
use ic_cdk_macros::query;

#[query]
fn latest_top_ups() -> TopUpsResponse {
    read_state(|state| state.data.canisters.latest_top_ups(200))
}
