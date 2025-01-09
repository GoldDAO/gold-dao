use crate::guards::caller_is_governance_principal;
use crate::model::neuron_system::sync_neurons;
use crate::state::read_state;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::manage_neuron::Args as ManageSnsNeuronArgs;
pub use gldt_stake_api_canister::manage_neuron::Response as ManageSnsNeuronResponse;
use ic_cdk::query;
use ic_cdk::update;
use sns_governance_canister::types::{manage_neuron::Command, ManageNeuron};
use tracing::{error, info};
use types::CanisterId;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manage_sns_neuron_validate(args: ManageSnsNeuronArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn manage_sns_neuron(args: ManageSnsNeuronArgs) -> ManageSnsNeuronResponse {
    let canister_id = read_state(|s| s.data.gld_sns_governance_canister_id);

    match manage_sns_neuron_impl(canister_id, args.neuron_id, args.command).await {
        Ok(ok) => {
            ic_cdk::spawn(async {
                let _ = sync_neurons().await;
            });
            ManageSnsNeuronResponse::Success(ok)
        }
        Err(err) => ManageSnsNeuronResponse::InternalError(err),
    }
}

pub(crate) async fn manage_sns_neuron_impl(
    canister_id: CanisterId,
    neuron_id: Vec<u8>,
    command: Command,
) -> Result<String, String> {
    let args = ManageNeuron {
        subaccount: neuron_id,
        command: Some(command),
    };

    match sns_governance_canister_c2c_client::manage_neuron(canister_id, &args).await {
        Ok(response) => {
            info!(
                "MANAGE NEURON :: Succesfully executed a neuron command: {:?}",
                response
            );
            Ok(("Succesfully executed a neuron command").to_string())
        }
        Err(e) => {
            error!(
                "MANAGE NEURON :: Failed to executed a neuron command: {:?}",
                e
            );
            Err(("Failed to executed a neuron command: {e:?}").to_string())
        }
    }
}
