use crate::guards::caller_is_governance_principal;
use crate::state::read_state;
use canister_tracing_macros::trace;
use ic_cdk::query;
use ic_cdk::update;
use sns_governance_canister::types::{manage_neuron::Command, ManageNeuron};
pub use sns_neuron_controller_api_canister::manage_sns_neuron::Args as ManageSnsNeuronArgs;
pub use sns_neuron_controller_api_canister::manage_sns_neuron::Response as ManageSnsNeuronResponse;
use sns_neuron_controller_api_canister::neuron_type::NeuronType;
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
    let canister_id = get_governance_canister_id(args.neuron_type);

    match manage_sns_neuron_impl(canister_id, args.neuron_id, args.command).await {
        Ok(ok) => ManageSnsNeuronResponse::Success(ok),
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
            info!("Succesfully executed a neuron command: {:?}", response);
            Ok(("Succesfully executed a neuron command").to_string())
        }
        Err(e) => {
            error!("Failed to executed a neuron command: {:?}", e);
            Err(("Failed to executed a neuron command: {e:?}").to_string())
        }
    }
}

pub fn get_governance_canister_id(neuron_type: NeuronType) -> CanisterId {
    match neuron_type {
        NeuronType::Ogy => read_state(|state| {
            state
                .data
                .neuron_managers
                .ogy
                .ogy_sns_governance_canister_id
        }),
        NeuronType::Wtn => read_state(|state| {
            state
                .data
                .neuron_managers
                .wtn
                .wtn_sns_governance_canister_id
        }),
    }
}
