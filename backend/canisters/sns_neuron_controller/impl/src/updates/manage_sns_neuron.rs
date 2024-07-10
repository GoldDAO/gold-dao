use crate::guards::caller_is_governance_principal;
use crate::state::read_state;
use candid::CandidType;
use ic_cdk::update;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::manage_neuron::Command;
use sns_governance_canister::types::ManageNeuron;
use tracing::error;
use tracing::info;
use types::CanisterId;

// #[derive(CandidType, Serialize, Deserialize, Clone)]
#[derive(CandidType, Deserialize, Clone)]
pub struct ManageSnsNeuronRequest {
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
    let canister_id = read_state(|state| state.data.sns_governance_canister_id);

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

    // OGY neurons receive their rewards in a custom way and need to claim their "real"
    // rewards via the OGY dashboard. It can also be done via direct calls to the sns_rewards
    // canister of OGY but it's different from "normal" neurons.

    // The current process to receive rewards is to once register the canister as the owner
    // in the sns_rewards canister via add_neuron_ownership and then claim the rewards each
    // time via claim_rewards of the sns_rewards canister.

    // The canister can query the balance of available OGY rewards via the ledger balance of
    // the subaccount on the sns_rewards canister (owner=ogy_sns_rewards_canister_id, subaccount=neuron_id)
    // Once the balance exceeds a certain threshold (e.g. 1 million OGY), the rewards can be
    // claimed and sent to the Gold DAO sns_rewards canister for distribution.
    // General maturity of the OGY neuron is automatically restaked and only the rewards from
    // the deflationary rewards are distributed to the gold dao neurons.

    match sns_governance_canister_c2c_client::manage_neuron(canister_id, &args).await {
        // TODO: Handle the response somehow. Can I implement Debug here?
        Ok(_response) => {
            info!("Succesfully executed a neuron command");
            return Ok(format!("Succesfully executed a neuron command"));
        }
        Err(e) => {
            error!("Failed to executed a neuron command: {:?}", e);
            return Err(format!("Failed to executed a neuron command: {e:?}"));
        }
    }
}
