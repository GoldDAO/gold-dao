use crate::types::neuron_manager::NeuronManager;
use crate::types::neuron_manager::{OgyManager, WtnManager};
// use crate::types::neuron_metrics::NeuronWithMetric;
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
    // FIXME
    // pub neuron_manager_metrics:
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub memory_used: MemorySize,
    // pub cycles_balance_in_tc: f64,
    pub cycles_balance: Cycles,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: Vec<Principal>,
    // NOTE: it seems to be not the best practice to store the manager struct inside the state, because then it makes all the mutations more complex and harder to handle
    pub neuron_managers: NeuronManagers,
    pub sns_rewards_canister_id: CanisterId,
}

impl Data {
    pub fn new(sns_rewards_canister_id: CanisterId) -> Self {
        Self {
            authorized_principals: vec![SNS_GOVERNANCE_CANISTER_ID],
            neuron_managers: NeuronManagers::default(),
            sns_rewards_canister_id,
        }
    }
}

// Think of how to not clone it each time. Probably, the best
// option would be to implement Rc<RefCell<T>> on top of this,
// but I'm not sure how it would match with current memory layout
#[derive(Serialize, Deserialize, Default)]
pub struct NeuronManagers {
    pub timestamp: TimestampMillis,
    pub ogy: OgyManager,
    pub wtn: WtnManager,
    pub others: Vec<Box<dyn NeuronManager>>,
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
