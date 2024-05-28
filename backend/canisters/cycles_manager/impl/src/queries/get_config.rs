use crate::guards::caller_is_governance_principal;
use crate::state::read_state;
use cycles_manager_canister::get_config::Response as GetConfigResponse;
use ic_cdk_macros::query;

#[query(guard = "caller_is_governance_principal", hidden = true)]
fn get_config() -> GetConfigResponse {
    read_state(|state| state.get_top_up_config())
}
