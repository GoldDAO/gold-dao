use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;

pub type Args = NeuronId;

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Ok(NeuronId),
    NeuronHotKeyAbsent,                    // No hotkeys found for neuron
    NeuronHotKeyInvalid, // Hotkeys exist but they don't match the caller's principal
    NeuronOwnerInvalid(Option<Principal>), // Neuron has a hotkey owned by a different caller
    NeuronDoesNotExist,
    InternalError(String),
}
