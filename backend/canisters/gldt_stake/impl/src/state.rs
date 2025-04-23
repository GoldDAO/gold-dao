use std::collections::{BTreeSet, HashMap};

use candid::{CandidType, Nat, Principal};
use canister_state_macros::canister_state;
use gldt_stake_common::{
    archive::{ArchiveCanister, ArchiveStatus},
    reward_tokens::{RewardTypes, TokenSymbol},
};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::Neuron;
use types::{BuildVersion, TimestampMillis};
use utils::{
    env::{CanisterEnv, Environment},
    memory::MemorySize,
};

use crate::{
    model::{
        archive_system::ArchiveSystem, neuron_system::NeuronSystem,
        proposal_system::ProposalSystem, reward_system::RewardSystem, stake_system::StakeSystem,
    },
    utils::TimeInterval,
};

canister_state!(RuntimeState);

#[derive(Default, Serialize, Deserialize)]
pub struct RuntimeState {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: Data,
}

pub type FeeAccount = Account;

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
            authorized_principals: self.data.authorized_principals.clone(),
            total_staked: format!("{:?}", self.data.stake_system.total_staked.0.clone()),
            total_active_stake_positions: self.data.stake_system.total_stake_positions,
            token_usd_values: self.data.stake_system.token_usd_values.clone(),
            genesis_datetime: self.data.stake_system.genesis_datetime,
            reward_types: self.data.stake_system.reward_types.clone(),
            reward_history: self.data.reward_system.reward_history.clone(),
            pending_reward_rounds: self.data.reward_system.rounds.len(),
            neurons: self.data.neuron_system.neurons.clone(),
            archive_canisters: self.data.archive_system.get_archive_canisters(),
            required_cycle_balance: format!(
                "{:?}",
                self.data.archive_system.required_cycle_balance.0.clone()
            ),
            archive_status: self.data.archive_system.archive_status.clone(),
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
    pub total_staked: String,
    pub total_active_stake_positions: usize,
    pub token_usd_values: HashMap<String, f64>,
    pub genesis_datetime: TimestampMillis,
    pub reward_types: RewardTypes,
    pub reward_history: HashMap<TokenSymbol, Nat>,
    pub pending_reward_rounds: usize,
    pub neurons: Vec<Neuron>,
    pub archive_canisters: Vec<ArchiveCanister>,
    pub required_cycle_balance: String,
    pub archive_status: ArchiveStatus,
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
    // ledgers and canister ids
    pub gldt_ledger_id: Principal,
    pub ogy_ledger_id: Principal,
    pub goldao_ledger_id: Principal,
    pub icp_ledger_id: Principal,
    pub goldao_sns_rewards_canister_id: Principal,
    pub goldao_sns_governance_canister_id: Principal,
    // authorized callers
    pub authorized_principals: Vec<Principal>,
    // storage for principals guard see guards.rs
    pub principal_guards: BTreeSet<Principal>,

    pub stake_system: StakeSystem,
    pub neuron_system: NeuronSystem,
    pub reward_system: RewardSystem,
    pub archive_system: ArchiveSystem,
    pub proposal_system: ProposalSystem,

    // cron job related
    /// the weekly interval that governs when neuron rewards are claimed from the sns_rewards canister
    pub reward_claim_interval: Option<TimeInterval>,
    /// flag to stop duplicate neuron reward claims
    pub is_reward_claim_in_progress: bool,
    /// bool flag to check if the reward allocation job is already running and to prevent duplicates
    pub is_reward_allocation_in_progress: bool,
    /// bool flag to check if the archive cron is running
    pub is_archive_cron_running: bool,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            gldt_ledger_id: Principal::anonymous(),
            ogy_ledger_id: Principal::anonymous(),
            goldao_ledger_id: Principal::anonymous(),
            icp_ledger_id: Principal::anonymous(),
            authorized_principals: vec![],
            stake_system: StakeSystem::default(),
            goldao_sns_rewards_canister_id: Principal::anonymous(),
            goldao_sns_governance_canister_id: Principal::anonymous(),
            neuron_system: NeuronSystem::default(),
            reward_system: RewardSystem::default(),
            reward_claim_interval: Some(TimeInterval {
                weekday: Some("Thursday".to_string()),
                start_hour: 12,
                end_hour: 13,
            }),
            is_reward_claim_in_progress: false,
            is_reward_allocation_in_progress: false,
            is_archive_cron_running: false,
            principal_guards: BTreeSet::new(),
            archive_system: ArchiveSystem::default(),
            proposal_system: ProposalSystem::default(),
        }
    }
}
