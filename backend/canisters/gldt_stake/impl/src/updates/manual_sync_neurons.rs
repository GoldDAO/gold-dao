use crate::guards::caller_is_governance_principal;
use crate::model::neuron_system::sync_neurons;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::manual_sync_neurons::{
    Args as ManualSyncNeuronsArgs, Response as ManualSyncNeuronsResponse,
};
use ic_cdk::query;
use ic_cdk::update;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manual_sync_neurons_validate(_args: ManualSyncNeuronsArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&_args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn manual_sync_neurons(_args: ManualSyncNeuronsArgs) -> ManualSyncNeuronsResponse {
    sync_neurons().await
}
