use std::collections::{ BTreeMap, HashMap };
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use candid::{ CandidType, Principal };
use canister_state_macros::canister_state;
use types::{ NeuronInfo, TimestampMillis, TokenInfo, TokenSymbol };
use utils::{
    consts::SNS_GOVERNANCE_CANISTER_ID,
    env::{ CanisterEnv, Environment },
    memory::MemorySize,
};

use crate::model::{
    maturity_history::MaturityHistory,
    payment_processor::PaymentProcessor,
    neuron_owners::NeuronOwnership,
};

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

    pub fn is_caller_governance_principal(&self) -> bool {
        let caller = self.env.caller();
        self.data.authorized_principals.contains(&caller)
    }

    pub fn set_is_synchronizing_neurons(&mut self, state: bool) {
        self.data.is_synchronizing_neurons = state;
    }

    pub fn get_is_synchronizing_neurons(&self) -> bool {
        self.data.is_synchronizing_neurons
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
    /// SNS governance canister
    pub sns_governance_canister: Principal,
    /// Stores the maturity information about each neuron
    pub neuron_maturity: BTreeMap<NeuronId, NeuronInfo>,
    /// Stores the mapping of each principal to its neurons
    pub principal_neurons: BTreeMap<Principal, Vec<NeuronId>>,
    /// Information about periodic synchronization
    pub sync_info: SyncInfo,
    /// The history of each neuron's maturity.
    pub maturity_history: MaturityHistory,
    /// owners of neurons
    pub neuron_owners: NeuronOwnership,
    /// Payment processor - responsible for queuing and processing rounds of payments
    pub payment_processor: PaymentProcessor,
    /// valid tokens and their associated ledger data
    pub tokens: HashMap<TokenSymbol, TokenInfo>,
    /// authorized Principals for guarded calls
    pub authorized_principals: Vec<Principal>,
    /// a boolean check for if we're currently synchronizing neuron data into the canister.
    pub is_synchronizing_neurons: bool,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            sns_governance_canister: SNS_GOVERNANCE_CANISTER_ID,
            neuron_maturity: BTreeMap::new(),
            principal_neurons: BTreeMap::new(),
            sync_info: SyncInfo::default(),
            maturity_history: MaturityHistory::default(),
            neuron_owners: NeuronOwnership::default(),
            payment_processor: PaymentProcessor::default(),
            tokens: HashMap::new(),
            authorized_principals: vec![SNS_GOVERNANCE_CANISTER_ID],
            is_synchronizing_neurons: false,
        }
    }
}
