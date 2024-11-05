use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub neuron_id: NeuronId,
    pub token: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    NeuronHotKeyAbsent,                    // No hotkeys found for neuron
    NeuronHotKeyInvalid, // Hotkeys exist but they don't match the caller's principal
    NeuronOwnerInvalid(Option<Principal>), // Neuron has a hotkey owned by a different caller
    NeuronNotClaimed,    // Nobody has claimed this neuron yet.
    NeuronDoesNotExist,
    InternalError(String),
    TransferFailed(String),
    TokenSymbolInvalid(String),
    Ok(bool),
}
