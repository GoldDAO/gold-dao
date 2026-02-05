use crate::model::neuron_system::NeuronSystem;
use crate::{model::payment_processor::PaymentProcessor, utils::TimeInterval};
use bity_ic_canister_state_macros::canister_state;
use bity_ic_types::BuildVersion;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use sns_rewards_api_canister::TokenRewardTypes;
use std::collections::HashMap;
use types::TimestampMillis;
use utils::{
    consts::SNS_GOVERNANCE_CANISTER_ID,
    env::{CanisterEnv, Environment},
    memory::MemorySize,
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
                version: self.env.version(),
                commit_hash: self.env.commit_hash().to_string(),
            },
            sns_governance_canister: self.data.sns_governance_canister,
            number_of_neurons: self.data.neuron_system.neuron_maturity.len(),
            sync_info: self.data.neuron_system.sync_info,
            authorized_principals: self.data.authorized_principals.clone(),
            reward_distribution_interval: self.data.reward_distribution_interval.clone(),
            neuron_sync_interval: self.data.neuron_sync_interval.clone(),
            registered_tokens: self
                .data
                .tokens
                .iter()
                .map(|(token, details)| {
                    format!(
                        "{:?} - id: {}, fee: {}, decimals: {}",
                        token, details.ledger_id, details.fee, details.decimals
                    )
                })
                .collect(),
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
    pub sync_info: SyncInfo,
    pub authorized_principals: Vec<Principal>,
    pub reward_distribution_interval: Option<TimeInterval>,
    pub neuron_sync_interval: Option<TimeInterval>,
    pub registered_tokens: Vec<String>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub memory_used: MemorySize,
    pub cycles_balance_in_tc: u128,
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
    pub neuron_system: NeuronSystem,
    /// Payment processor - responsible for queuing and processing rounds of payments
    pub payment_processor: PaymentProcessor,
    /// valid tokens and their associated ledger data
    pub tokens: TokenRewardTypes,
    /// authorized Principals for guarded calls
    pub authorized_principals: Vec<Principal>,
    /// a boolean check for if we're currently synchronizing neuron data into the canister.
    pub is_synchronizing_neurons: bool,
    /// The weekly interval for which a reward distribution occurs
    pub reward_distribution_interval: Option<TimeInterval>,
    /// An internal check if the distribution is running
    pub reward_distribution_in_progress: Option<bool>,
    /// An internal check if the GLDT distribution is running
    pub gldt_distribution_in_progress: Option<bool>,
    /// The daily interval for which a neuron sync occurs
    pub neuron_sync_interval: Option<TimeInterval>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            sns_governance_canister: SNS_GOVERNANCE_CANISTER_ID,
            neuron_system: NeuronSystem::default(),
            payment_processor: PaymentProcessor::default(),
            tokens: HashMap::new(),
            authorized_principals: vec![SNS_GOVERNANCE_CANISTER_ID],
            is_synchronizing_neurons: false,
            reward_distribution_interval: Some(TimeInterval::default()),
            reward_distribution_in_progress: Some(false),
            gldt_distribution_in_progress: Some(false),
            neuron_sync_interval: Some(TimeInterval {
                weekday: None,
                start_hour: 9,
                end_hour: 11,
            }),
        }
    }
}
