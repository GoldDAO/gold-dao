use candid::CandidType;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::Neuron;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct NeuronList {
    pub ogy_neurons: Vec<Neuron>,
    pub wtn_neurons: Vec<Neuron>,
}
