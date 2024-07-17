use crate::types::neuron_manager::OgyManager;
use crate::types::neuron_manager::WtnManager;
use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
use ledger_utils::principal_to_legacy_account_id;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::Neuron;
use types::{RewardsRecipientList, TimestampMillis};
use utils::{
    consts::SNS_GOVERNANCE_CANISTER_ID,
    env::{CanisterEnv, Environment},
    memory::MemorySize,
};

use crate::types::{
    neuron_metrics::NeuronWithMetric, outstanding_payments::OutstandingPaymentsList,
};

canister_state!(RuntimeState);

#[derive(Serialize, Deserialize)]
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
            canister_default_account_id: principal_to_legacy_account_id(
                self.env.canister_id(),
                None,
            )
            .to_string(),
            authorized_principals: self.data.authorized_principals.clone(),
            rewards_recipients: self.data.rewards_recipients.clone(),
            outstanding_payments: self.data.outstanding_payments.clone(),
        }
    }

    pub fn is_caller_governance_principal(&self) -> bool {
        let caller = self.env.caller();
        self.data.authorized_principals.contains(&caller)
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub canister_default_account_id: String,
    pub authorized_principals: Vec<Principal>,
    pub rewards_recipients: RewardsRecipientList,
    pub outstanding_payments: OutstandingPaymentsList,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: Vec<Principal>,
    // NOTE: it seems to be not the best practice to store the manager struct inside the state, because then it makes all the mutations more complex and harder to handle
    pub neuron_managers: NeuronManagers,
    pub rewards_recipients: RewardsRecipientList,
    pub outstanding_payments: OutstandingPaymentsList,
}

impl Data {
    pub fn new() -> Self {
        Self {
            rewards_recipients: RewardsRecipientList::empty(),
            authorized_principals: vec![SNS_GOVERNANCE_CANISTER_ID],
            neuron_managers: NeuronManagers::default(),
            outstanding_payments: OutstandingPaymentsList::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct NeuronManagers {
    pub timestamp: TimestampMillis,
    pub ogy: OgyManager,
    pub wtn: WtnManager,
    // TODO: impl also other neuron manager
    pub others: Vec<u64>,
}

impl NeuronManagers {
    pub fn get_neurons(&self) -> NeuronList {
        NeuronList {
            ogy_neurons: self.ogy.neurons.all_neurons.clone(),
            wtn_neurons: self.wtn.neurons.all_neurons.clone(),
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct NeuronList {
    ogy_neurons: Vec<Neuron>,
    wtn_neurons: Vec<Neuron>,
}
