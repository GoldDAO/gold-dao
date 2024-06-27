use crate::state::read_state;
pub use cycles_manager_api_canister::get_config::Response as GetConfigResponse;
use ic_cdk_macros::query;

#[query]
fn get_config() -> GetConfigResponse {
    read_state(|state| state.get_config())
}
