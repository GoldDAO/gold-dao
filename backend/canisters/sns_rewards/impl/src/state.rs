use std::collections::BTreeMap;
use serde::Deserialize;
use sns_governance_canister::types::NeuronId;
use candid::{ CandidType, Principal };
use canister_state_macros::canister_state;

canister_state!(RuntimeState);

/// The maturity information about a neuron
#[derive(CandidType, Clone, Deserialize)]
pub struct NeuronInfo {
    pub last_synced_maturity: u64,
    pub accumulated_maturity: u64,
}

#[derive(CandidType, Deserialize)]
pub struct RuntimeState {
    /// Stores the maturity information about each neuron
    pub neuron_maturity: BTreeMap<NeuronId, NeuronInfo>,
    /// Stores the mapping of each principal to its neurons
    pub principal_neurons: BTreeMap<Principal, Vec<NeuronId>>,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            neuron_maturity: BTreeMap::new(),
            principal_neurons: BTreeMap::new(),
        }
    }
}
