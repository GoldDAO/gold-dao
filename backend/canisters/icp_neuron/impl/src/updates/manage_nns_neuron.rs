use crate::ecdsa::make_canister_call_via_ecdsa;
use crate::guards::caller_is_governance_principal;
use crate::state::mutate_state;
use candid::CandidType;
use canister_tracing_macros::trace;
use ic_cdk::update;
use nns_governance_canister::types::manage_neuron::Command;
use nns_governance_canister::types::ManageNeuron;
use serde::{ Deserialize, Serialize };

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ManageNnsNeuronRequest {
    pub neuron_id: u64,
    pub command: Command,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ManageNnsNeuronResponse {
    Success(String),
    InternalError(String),
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn manage_nns_neuron(args: ManageNnsNeuronRequest) -> ManageNnsNeuronResponse {
    manage_nns_neuron_impl(args.neuron_id, args.command).await
}

pub(crate) async fn manage_nns_neuron_impl(
    neuron_id: u64,
    command: Command
) -> ManageNnsNeuronResponse {
    let nonce: Vec<u8>;
    if let Ok((rand_bytes,)) = ic_cdk::api::management_canister::main::raw_rand().await {
        nonce = rand_bytes;
    } else {
        return ManageNnsNeuronResponse::InternalError("Unable to initialise nonce.".to_string());
    }
    let request = mutate_state(|state| {
        state.prepare_canister_call_via_ecdsa(
            state.data.nns_governance_canister_id,
            "manage_neuron".to_string(),
            ManageNeuron::new(neuron_id, command),
            Some(nonce)
        )
    });

    match make_canister_call_via_ecdsa(request).await {
        Ok(response) => ManageNnsNeuronResponse::Success(response),
        Err(error) => ManageNnsNeuronResponse::InternalError(error),
    }
}
