use crate::guards::caller_is_governance_principal;
use crate::types::neuron_manager::NeuronType;
use candid::CandidType;
use ic_cdk::update;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::manage_neuron::Command;
use sns_governance_canister::types::ManageNeuron;
use tracing::error;
use tracing::info;
use types::CanisterId;

#[derive(CandidType, Deserialize, Clone)]
pub struct ManageSnsNeuronRequest {
    pub neuron_type: NeuronType,
    pub neuron_id: Vec<u8>,
    pub command: Command,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ManageSnsNeuronResponse {
    Success(String),
    InternalError(String),
}

#[update(guard = "caller_is_governance_principal")]
// #[trace]
async fn manage_sns_neuron(args: ManageSnsNeuronRequest) -> ManageSnsNeuronResponse {
    let canister_id = args.neuron_type.get_governance_canister_id();

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
        // TODO: Handle the response somehow. Can I implement Debug here?
        Ok(_response) => {
            info!("Succesfully executed a neuron command");
            Ok(("Succesfully executed a neuron command").to_string())
        }
        Err(e) => {
            error!("Failed to executed a neuron command: {:?}", e);
            Err(("Failed to executed a neuron command: {e:?}").to_string())
        }
    }
}
