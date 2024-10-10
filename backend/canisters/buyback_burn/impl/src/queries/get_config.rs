use crate::guards::caller_is_governance_principal;
use crate::state::read_state;
use buyback_burn_api::get_config::Response as GetConfigResponse;
use ic_cdk_macros::query;

#[query(guard = "caller_is_governance_principal", hidden = true)]
fn get_config() -> GetConfigResponse {
    read_state(|state| state.get_config())
}
