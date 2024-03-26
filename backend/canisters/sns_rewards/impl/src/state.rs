use std::collections::BTreeMap;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use candid::{ CandidType, Principal };
use canister_state_macros::canister_state;
use types::{ NeuronInfo, TimestampMillis };
use utils::{
    consts::SNS_GOVERNANCE_CANISTER_ID,
    env::{ CanisterEnv, Environment },
    memory::MemorySize,
};

use crate::model::{ maturity_history::MaturityHistory, neuron_owners::NeuronOwners };

canister_state!(RuntimeState);

#[derive(Default, Serialize, Deserialize)]
pub struct RuntimeState {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: CanisterEnv, data: Data) -> Self {
        Self { env, data }
    }
    pub fn metrics(&self) -> Metrics {
        Metrics {
            canister_info: CanisterInfo {
                now: self.env.now(),
                test_mode: self.env.is_test_mode(),
                memory_used: MemorySize::used(),
                cycles_balance_in_tc: self.env.cycles_balance_in_tc(),
            },
            sns_governance_canister: self.data.sns_governance_canister,
            number_of_neurons: self.data.neuron_maturity.len(),
            number_of_owners: self.data.principal_neurons.len(),
            sync_info: self.data.sync_info,
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
}

#[derive(CandidType, Deserialize, Serialize, Clone, Copy, Default)]
pub struct SyncInfo {
    pub last_synced_start: TimestampMillis,
    pub last_synced_end: TimestampMillis,
    pub last_synced_number_of_neurons: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    /// SNS governance cansiter
    pub sns_governance_canister: Principal,
    /// Stores the maturity information about each neuron
    pub neuron_maturity: BTreeMap<NeuronId, NeuronInfo>,
    /// Stores the mapping of each principal to its neurons
    pub principal_neurons: BTreeMap<Principal, Vec<NeuronId>>,
    /// Information about periodic synchronisation
    pub sync_info: SyncInfo,
    /// The history of each neuron's maturity.
    pub maturity_history: MaturityHistory,
    /// owners of neurons
    pub neuron_owners: NeuronOwners,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            sns_governance_canister: SNS_GOVERNANCE_CANISTER_ID,
            neuron_maturity: BTreeMap::new(),
            principal_neurons: BTreeMap::new(),
            sync_info: SyncInfo::default(),
            maturity_history: MaturityHistory::default(),
            neuron_owners: NeuronOwners::default(),
        }
    }
}
