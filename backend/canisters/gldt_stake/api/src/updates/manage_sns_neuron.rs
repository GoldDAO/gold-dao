use candid::CandidType;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::manage_neuron::Command;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub neuron_id: Vec<u8>,
    pub command: Command,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(String),
    InternalError(String),
}
