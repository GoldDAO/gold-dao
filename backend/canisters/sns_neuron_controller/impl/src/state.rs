use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
use ledger_utils::principal_to_legacy_account_id;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::Neuron;
use types::{CanisterId, RewardsRecipientList, TimestampMillis};
use utils::{
    consts::{ICP_LEDGER_CANISTER_ID, SNS_GOVERNANCE_CANISTER_ID, SNS_REWARDS_CANISTER_ID},
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
            neurons: self.data.get_ogy_neuron_list(),
            ogy_sns_governance_canister_id: self.data.ogy_sns_governance_canister_id,
            ogy_sns_ledger_canister_id: self.data.ogy_sns_ledger_canister_id,
            ogy_sns_rewards_canister_id: self.data.ogy_sns_rewards_canister_id,
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
    pub ogy_sns_governance_canister_id: CanisterId,
    pub ogy_sns_ledger_canister_id: CanisterId,
    pub ogy_sns_rewards_canister_id: CanisterId,
    pub rewards_recipients: RewardsRecipientList,
    pub neurons: NeuronList,
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
    // TODO: take out all the OGY parameters into separate struct and try to make it abstract while adding WTN: sns_governance_canister_id, sns_ledger_canister_id
    pub ogy_neurons: Neurons,
    pub ogy_sns_governance_canister_id: Principal,
    pub ogy_sns_ledger_canister_id: Principal,
    pub ogy_sns_rewards_canister_id: CanisterId,
    pub rewards_recipients: RewardsRecipientList,
    pub outstanding_payments: OutstandingPaymentsList,
}

impl Data {
    pub fn new() -> Self {
        Self {
            rewards_recipients: RewardsRecipientList::empty(),
            authorized_principals: vec![SNS_GOVERNANCE_CANISTER_ID],
            ogy_neurons: Neurons::default(),
            // FIXME: change this value to valid:
            ogy_sns_ledger_canister_id: SNS_GOVERNANCE_CANISTER_ID,
            // FIXME: change this value to valid:
            ogy_sns_governance_canister_id: SNS_GOVERNANCE_CANISTER_ID,
            outstanding_payments: OutstandingPaymentsList::default(),
            ogy_sns_rewards_canister_id: SNS_REWARDS_CANISTER_ID,
        }
    }

    pub fn get_ogy_neuron_list(&self) -> NeuronList {
        NeuronList {
            active: self
                .ogy_neurons
                .active_neurons
                .iter()
                .map(|n| NeuronWithMetric::from(n.clone()))
                .collect(),
            // TODO: think of .clone()
            spawning: self
                .ogy_neurons
                .spawning_neurons
                .iter()
                .filter_map(|n| n.id.as_ref().map(|id| id.id.clone()))
                .collect(),
            disbursed: self.ogy_neurons.disbursed_neurons.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Neurons {
    pub timestamp: TimestampMillis,
    pub all_neurons: Vec<Neuron>,
    // TODO: think more of how to classify sns neurons and validate them
    pub active_neurons: Vec<Neuron>,
    pub spawning_neurons: Vec<Neuron>,
    pub disbursed_neurons: Vec<u64>,
}

#[derive(CandidType, Serialize)]
pub struct NeuronList {
    active: Vec<NeuronWithMetric>,
    spawning: Vec<Vec<u8>>,
    disbursed: Vec<u64>,
}
