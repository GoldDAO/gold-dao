use candid::CandidType;
use nns_governance_canister::types::manage_neuron::Command;

use serde::{Deserialize, Serialize};

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
