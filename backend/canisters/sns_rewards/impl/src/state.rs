use std::collections::BTreeMap;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use candid::{ CandidType, Principal };
use canister_state_macros::canister_state;
use types::{ Cycles, NeuronInfo, TimestampMillis };
use utils::{ env::{ CanisterEnv, Environment }, memory::MemorySize };

use crate::model::maturity_history::MaturityHistory;

canister_state!(RuntimeState);

#[derive(Serialize, Deserialize)]
pub struct RuntimeState {
    // These are maintained via pre_ and post_upgrade hooks

    /// Runtime environment
    pub env: CanisterEnv,
    /// SNS governance cansiter
    pub sns_governance_canister: Principal,
    /// Stores the maturity information about each neuron
    pub neuron_maturity: BTreeMap<NeuronId, NeuronInfo>,
    /// Stores the mapping of each principal to its neurons
    pub principal_neurons: BTreeMap<Principal, Vec<NeuronId>>,
    /// Information about periodic synchronisation
    pub sync_info: SyncInfo,

    // These are maintained directly via the stable memory

    /// The history of each neuron's maturity.
    pub maturity_history: MaturityHistory,
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
            env: CanisterEnv::default(),
            sns_governance_canister: Principal::anonymous(),
            neuron_maturity: BTreeMap::new(),
            principal_neurons: BTreeMap::new(),
            sync_info: SyncInfo::default(),
            maturity_history: MaturityHistory::default(),
        }
    }
    pub fn metrics(&self) -> Metrics {
        Metrics {
            canister_info: CanisterInfo {
                now: self.env.now(),
                test_mode: self.env.test_mode(),
                memory_used: MemorySize::used(),
                cycles_balance_in_tc: self.env.cycles_balance_in_tc(),
            },
            sns_governance_canister: self.sns_governance_canister,
            number_of_neurons: self.neuron_maturity.len(),
            number_of_owners: self.principal_neurons.len(),
            sync_info: self.sync_info,
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub sns_governance_canister: Principal,
    pub number_of_neurons: usize,
    pub number_of_owners: usize,
    pub sync_info: SyncInfo,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: f64,
    // pub wasm_version: BuildVersion,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Copy)]
pub struct SyncInfo {
    pub last_synced_start: TimestampMillis,
    pub last_synced_end: TimestampMillis,
    pub last_synced_number_of_neurons: usize,
}

impl Default for SyncInfo {
    fn default() -> Self {
        Self {
            last_synced_start: 0,
            last_synced_end: 0,
            last_synced_number_of_neurons: 0,
        }
    }
}
