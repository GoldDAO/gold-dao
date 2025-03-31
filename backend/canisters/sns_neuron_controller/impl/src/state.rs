use crate::types::neuron_manager::NeuronConfig;
use crate::types::neuron_manager::NeuronManager;
use crate::types::neuron_manager::NeuronManagerEnum;
use crate::types::neuron_metrics::NeuronWithMetric;
use crate::types::{OgyManager, WtnManager};
use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
use serde::{Deserialize, Serialize};
use sns_neuron_controller_api_canister::init::OgyManagerConfig;
use sns_neuron_controller_api_canister::init::WtnManagerConfig;
use sns_neuron_controller_api_canister::list_neurons_type::NeuronList;
use sns_neuron_controller_api_canister::neuron_type::NeuronType;
use types::BuildVersion;
use types::CanisterId;
use types::TimestampMillis;
use utils::{
    env::{CanisterEnv, Environment},
    memory::MemorySize,
};

canister_state!(RuntimeState);

#[derive(Serialize, Deserialize)]
pub struct RuntimeState {
    pub env: CanisterEnv,
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
                version: self.env.version(),
                commit_hash: self.env.commit_hash().to_string(),
                test_mode: self.env.is_test_mode(),
                memory_used: MemorySize::used(),
                cycles_balance_in_tc: self.env.cycles_balance_in_tc(),
            },
            authorized_principals: self.data.authorized_principals.clone(),
            sns_rewards_canister_id: self.data.sns_rewards_canister_id,
            ogy_neuron_manager_config: OgyManagerConfig {
                ogy_sns_governance_canister_id: self
                    .data
                    .neuron_managers
                    .ogy
                    .get_sns_governance_canister_id(),
                ogy_sns_ledger_canister_id: self
                    .data
                    .neuron_managers
                    .ogy
                    .get_sns_ledger_canister_id(),
                ogy_sns_rewards_canister_id: self
                    .data
                    .neuron_managers
                    .ogy
                    .get_sns_rewards_canister_id(),
                ogy_rewards_threshold: self.data.neuron_managers.ogy.ogy_rewards_threshold.clone(),
            },
            ogy_neuron_manager_metrics: self.data.neuron_managers.ogy.get_neuron_metrics(),
            wtn_neuron_manager_config: WtnManagerConfig {
                wtn_sns_governance_canister_id: self
                    .data
                    .neuron_managers
                    .wtn
                    .get_sns_governance_canister_id(),
                wtn_sns_ledger_canister_id: self
                    .data
                    .neuron_managers
                    .wtn
                    .get_sns_ledger_canister_id(),
                icp_ledger: self.data.neuron_managers.wtn.icp_ledger,
                icp_rewards_threshold: self.data.neuron_managers.wtn.icp_rewards_threshold.clone(),
                wtn_rewards_threshold: self.data.neuron_managers.wtn.wtn_rewards_threshold.clone(),
            },
            wtn_neuron_manager_metrics: self.data.neuron_managers.wtn.get_neuron_metrics(),
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
    pub ogy_neuron_manager_config: OgyManagerConfig,
    pub ogy_neuron_manager_metrics: Vec<NeuronWithMetric>,
    pub wtn_neuron_manager_config: WtnManagerConfig,
    pub wtn_neuron_manager_metrics: Vec<NeuronWithMetric>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: f64,
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
        ogy_manager_config: OgyManagerConfig,
        wtn_manager_config: WtnManagerConfig,
        sns_rewards_canister_id: CanisterId,
        now: TimestampMillis,
    ) -> Self {
        Self {
            authorized_principals,
            neuron_managers: NeuronManagers::init(ogy_manager_config, wtn_manager_config, now),
            sns_rewards_canister_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NeuronManagers {
    pub now: TimestampMillis,
    pub ogy: OgyManager,
    pub wtn: WtnManager,
}

impl NeuronManagers {
    pub fn init(
        ogy_manager_config: OgyManagerConfig,
        wtn_manager_config: WtnManagerConfig,
        now: TimestampMillis,
    ) -> Self {
        Self {
            now,
            ogy: ogy_manager_config.into(),
            wtn: wtn_manager_config.into(),
        }
    }

    pub fn get_neurons(&self) -> NeuronList {
        NeuronList {
            ogy_neurons: self.ogy.neurons.all_neurons.clone(),
            wtn_neurons: self.wtn.neurons.all_neurons.clone(),
        }
    }

    pub fn get_neuron_manager(&self, neuron_type: NeuronType) -> Option<NeuronManagerEnum> {
        match neuron_type {
            NeuronType::OGY => Some(NeuronManagerEnum::OgyManager(self.ogy.clone())),
            NeuronType::WTN => Some(NeuronManagerEnum::WtnManager(self.wtn.clone())),
        }
    }
}
