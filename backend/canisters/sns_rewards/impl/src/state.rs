use std::collections::BTreeMap;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use candid::{ CandidType, Principal };
use canister_state_macros::canister_state;
use types::TimestampMillis;

canister_state!(RuntimeState);

/// The maturity information about a neuron
#[derive(CandidType, Clone, Deserialize)]
pub struct NeuronInfo {
    pub last_synced_maturity: u64,
    pub accumulated_maturity: u64,
}

#[derive(CandidType, Deserialize)]
pub struct RuntimeState {
    /// SNS governance cansiter
    pub sns_governance_canister: Principal,
    /// Stores the maturity information about each neuron
    pub neuron_maturity: BTreeMap<NeuronId, NeuronInfo>,
    /// Stores the mapping of each principal to its neurons
    pub principal_neurons: BTreeMap<Principal, Vec<NeuronId>>,
    /// Debug data about synchronisation
    pub debug_data: DebugData,
}

impl RuntimeState {
    pub fn new(sns_governance_canister: Principal) -> Self {
        Self {
            sns_governance_canister,
            ..Self::default()
        }
    }
    pub fn default() -> Self {
        Self {
            sns_governance_canister: Principal::anonymous(),
            neuron_maturity: BTreeMap::new(),
            principal_neurons: BTreeMap::new(),
            debug_data: DebugData::default(),
        }
    }
    pub fn metrics(&self) -> Metrics {
        Metrics {
            sns_governance_canister: self.sns_governance_canister,
            number_of_neurons: self.neuron_maturity.len(),
            number_of_owners: self.principal_neurons.len(),
            last_synced_start: self.debug_data.last_synced_start,
            last_synced_end: self.debug_data.last_synced_end,
            last_synced_number_of_neurons: self.debug_data.last_synced_number_of_neurons,
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub sns_governance_canister: Principal,
    pub number_of_neurons: usize,
    pub number_of_owners: usize,
    pub last_synced_start: TimestampMillis,
    pub last_synced_end: TimestampMillis,
    pub last_synced_number_of_neurons: usize,
}

#[derive(CandidType, Deserialize)]
pub struct DebugData {
    pub last_synced_start: TimestampMillis,
    pub last_synced_end: TimestampMillis,
    pub last_synced_number_of_neurons: usize,
}

impl DebugData {
    fn default() -> Self {
        Self {
            last_synced_start: 0,
            last_synced_end: 0,
            last_synced_number_of_neurons: 0,
        }
    }
}
