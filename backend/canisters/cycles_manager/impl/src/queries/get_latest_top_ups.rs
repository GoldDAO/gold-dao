use crate::guards::caller_is_governance_principal;
use crate::state::read_state;
use crate::TopUpsResponse;
use ic_cdk_macros::query;

#[query(guard = "caller_is_governance_principal", hidden = true)]
fn latest_top_ups() -> TopUpsResponse {
    read_state(|state| state.data.canisters.latest_top_ups(200))
}
