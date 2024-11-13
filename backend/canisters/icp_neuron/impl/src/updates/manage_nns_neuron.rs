use crate::guards::caller_is_governance_principal;
use crate::{ecdsa::make_canister_call_via_ecdsa, state::read_state};
use candid::CandidType;
use canister_tracing_macros::trace;
use ic_cdk::{query, update};
use nns_governance_canister::types::manage_neuron::Command;
use nns_governance_canister::types::ManageNeuron;
use serde::{Deserialize, Serialize};
use utils::rand::generate_rand_byte_array;

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

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manage_nns_neuron_validate(args: ManageNnsNeuronRequest) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
async fn manage_nns_neuron(args: ManageNnsNeuronRequest) -> ManageNnsNeuronResponse {
    match manage_nns_neuron_impl(args.neuron_id, args.command).await {
        Ok(ok) => ManageNnsNeuronResponse::Success(ok),
        Err(err) => ManageNnsNeuronResponse::InternalError(err),
    }
}

pub(crate) async fn manage_nns_neuron_impl(
    neuron_id: u64,
    command: Command,
) -> Result<String, String> {
    let nonce = generate_rand_byte_array().await?;

    let request = read_state(|state| {
        state.prepare_canister_call_via_ecdsa(
            state.data.nns_governance_canister_id,
            "manage_neuron".to_string(),
            ManageNeuron::new(neuron_id, command),
            Some(nonce),
        )
    })?;

    make_canister_call_via_ecdsa(request).await
}
