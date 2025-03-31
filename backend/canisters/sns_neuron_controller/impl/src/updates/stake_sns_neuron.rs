use crate::guards::caller_is_governance_principal;
use crate::state::read_state;
use crate::types::neuron_manager::NeuronManager;
use canister_tracing_macros::trace;
use ic_cdk::{query, update};
pub use sns_neuron_controller_api_canister::stake_sns_neuron::Args as StakeSnsNeuronArgs;
pub use sns_neuron_controller_api_canister::stake_sns_neuron::Response as StakeSnsNeuronResponse;
use tracing::error;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn stake_sns_neuron_validate(args: StakeSnsNeuronArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn stake_sns_neuron(args: StakeSnsNeuronArgs) -> StakeSnsNeuronResponse {
    let neuron_manager =
        read_state(|s| s.data.neuron_managers.get_neuron_manager(args.neuron_type));

    match neuron_manager {
        Some(manager) => match manager
            .stake_sns_neuron(args.amount, args.add_disolve_delay)
            .await
        {
            Ok(neuron_id) => StakeSnsNeuronResponse::Success(neuron_id),
            Err(error) => {
                error!(error);
                StakeSnsNeuronResponse::InternalError(error)
            }
        },
        None => StakeSnsNeuronResponse::InternalError("Neuron manager not found".to_string()),
    }
}
