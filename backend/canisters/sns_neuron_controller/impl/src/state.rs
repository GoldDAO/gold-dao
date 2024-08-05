use crate::types::neuron_manager::NeuronConfig;
use crate::types::neuron_manager::Neurons;
use crate::types::neuron_metrics::NeuronWithMetric;
use crate::types::{OgyManager, WtnManager};
use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::Neuron;
use types::CanisterId;
use types::Cycles;
use types::TimestampMillis;
use utils::{
    consts::SNS_GOVERNANCE_CANISTER_ID,
    env::{CanisterEnv, Environment},
    memory::MemorySize,
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
                cycles_balance: self.env.cycles_balance(),
            },

            authorized_principals: self.data.authorized_principals.clone(),
            sns_rewards_canister_id: self.data.sns_rewards_canister_id,
            ogy_neuron_manager_metrics: self
                .data
                .neuron_managers
                .ogy
                .get_neurons()
                .all_neurons
                .iter()
                .map(|n| NeuronWithMetric::from(n.clone()))
                .collect(),
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
    pub authorized_principals: Vec<Principal>,
    pub sns_rewards_canister_id: Principal,
    pub ogy_neuron_manager_metrics: Vec<NeuronWithMetric>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    pub cycles_balance: Cycles,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: Vec<Principal>,
    pub neuron_managers: NeuronManagers,
    pub sns_rewards_canister_id: CanisterId,
}

impl Data {
    pub fn new(
        authorized_principals: Vec<Principal>,
        ogy_sns_governance_canister_id: CanisterId,
        ogy_sns_ledger_canister_id: CanisterId,
        ogy_sns_rewards_canister_id: CanisterId,
        sns_rewards_canister_id: CanisterId,
        now: TimestampMillis,
    ) -> Self {
        Self {
            authorized_principals,
            neuron_managers: NeuronManagers::init(
                ogy_sns_governance_canister_id,
                ogy_sns_ledger_canister_id,
                ogy_sns_rewards_canister_id,
                now,
            ),
            sns_rewards_canister_id,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct NeuronManagers {
    pub now: TimestampMillis,
    pub ogy: OgyManager,
    pub wtn: WtnManager,
}

impl NeuronManagers {
    pub fn init(
        ogy_sns_governance_canister_id: CanisterId,
        ogy_sns_ledger_canister_id: CanisterId,
        ogy_sns_rewards_canister_id: CanisterId,
        now: TimestampMillis,
    ) -> Self {
        Self {
            now,
            ogy: OgyManager {
                ogy_sns_governance_canister_id,
                ogy_sns_ledger_canister_id,
                ogy_sns_rewards_canister_id,
                neurons: Neurons::default(),
            },
            wtn: WtnManager::default(),
        }
    }

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
