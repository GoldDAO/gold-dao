use crate::model::allocated_rewards_pool::AllocatedRewardsPool;
use crate::model::allocated_rewards_pool::AllocatedRewardsPoolMetrics;
use crate::model::analytics_system::AnalyticsSystem;
use crate::model::processing_rewards_pool::ProcessingRewardsPool;
use crate::model::unallocated_rewards_pool::*;
use bity_ic_icrc3::transaction::TransactionType;
use candid::{CandidType, Nat, Principal};
use canister_state_macros::canister_state;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::Neuron;
use std::collections::{BTreeSet, HashMap};
use types::TokenSymbol;
use types::{BuildVersion, TimestampMillis};
use utils::{
    env::{CanisterEnv, Environment},
    memory::MemorySize,
};

use crate::{
    model::{
        neuron_system::NeuronSystem, proposal_system::ProposalSystem, stake_system::StakeSystem,
    },
    utils::TimeInterval,
};

canister_state!(RuntimeState);
use bity_ic_icrc3_macros::icrc3_state;
icrc3_state!();

#[derive(Default, Serialize, Deserialize)]
pub struct RuntimeState {
    pub env: CanisterEnv,
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
            whitelist: self.data.whitelist.clone(),

            total_staked: format!("{:?}", self.data.stake_system.total_staked.0.clone()),
            // daily_weighted_staked_gldt: self.data.stake_system.daily_weighted_staked_gldt.clone(),
            daily_apy_timestamp: self.data.stake_system.cached_daily_timestamp,
            total_active_stake_positions: self.data.stake_system.active_stake_positions(),
            token_usd_values: self.data.stake_system.token_usd_values.clone(),
            genesis_datetime: self.data.stake_system.genesis_datetime,
            reward_types: self.data.stake_system.reward_types.clone(),
            reward_history: self.data.allocated_rewards_pool.reward_history.clone(),
            neurons: self.data.neuron_system.neurons.clone(),
            pending_fee_transfer_amount: self.data.stake_system.pending_fee_transfer_amount.clone(),

            gldt_ledger_id: self.data.gldt_ledger_id,
            goldao_ledger_id: self.data.goldao_ledger_id,
            goldao_sns_rewards_canister_id: self.data.goldao_sns_rewards_canister_id,
            goldao_sns_governance_canister_id: self.data.goldao_sns_governance_canister_id,
            unallocated_rewards_pool: self.data.unallocated_rewards_pool.clone(),
            processing_rewards_pool: self.data.processing_rewards_pool.clone(),
            allocated_rewards_pool: self.data.allocated_rewards_pool.clone().into(),
        }
    }

    pub fn is_canister_at_capacity_heap(&self) -> Result<(), String> {
        let memory = MemorySize::used();
        let used_heap = memory.heap as u128;
        let limit = 2 * 1024 * 1024 * 1024_u128; // 2GB

        if used_heap > limit {
            return Err(format!(
                "The canister is using {used_heap} bytes. limit is {limit}"
            ));
        }

        Ok(())
    }

    pub fn is_caller_governance_principal(&self) -> bool {
        let caller = self.env.caller();
        self.data.authorized_principals.contains(&caller)
    }

    pub fn is_caller_whitelisted(&self) -> bool {
        let caller = self.env.caller();
        self.data.whitelist.contains(&caller)
    }
}

#[derive(CandidType, Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub authorized_principals: Vec<Principal>,
    pub whitelist: Vec<Principal>,
    pub total_staked: String,
    pub total_active_stake_positions: usize,
    // pub daily_weighted_staked_gldt: HashMap<TimestampMillis, Nat>,
    pub daily_apy_timestamp: TimestampMillis,
    pub token_usd_values: HashMap<TokenSymbol, f64>,
    pub genesis_datetime: TimestampMillis,
    pub reward_history: HashMap<TokenSymbol, Nat>,
    pub neurons: Vec<Neuron>,
    pub reward_types: BTreeSet<TokenSymbol>,
    pub pending_fee_transfer_amount: Nat,

    // ledgers and canister ids
    pub gldt_ledger_id: Principal,
    pub goldao_ledger_id: Principal,
    pub goldao_sns_rewards_canister_id: Principal,
    pub goldao_sns_governance_canister_id: Principal,

    // rewards pools
    pub unallocated_rewards_pool: UnallocatedRewardsPool,
    pub processing_rewards_pool: ProcessingRewardsPool,
    pub allocated_rewards_pool: AllocatedRewardsPoolMetrics,
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

#[derive(Serialize, Deserialize)]
pub struct Data {
    // ledgers and canister ids
    pub gldt_ledger_id: Principal,
    pub goldao_ledger_id: Principal,
    pub goldao_sns_rewards_canister_id: Principal,
    pub goldao_sns_governance_canister_id: Principal,

    // authorized callers
    pub authorized_principals: Vec<Principal>,
    pub whitelist: Vec<Principal>,
    // storage for principals guard see guards.rs
    pub principal_guards: BTreeSet<Principal>,
    pub stake_system: StakeSystem,
    pub neuron_system: NeuronSystem,
    pub proposal_system: ProposalSystem,
    pub analytics_system: AnalyticsSystem,

    // rewards pools
    pub unallocated_rewards_pool: UnallocatedRewardsPool,
    pub processing_rewards_pool: ProcessingRewardsPool,
    pub allocated_rewards_pool: AllocatedRewardsPool,

    // cron job related
    pub reward_claim_interval: Option<TimeInterval>,
    pub allocate_rewards_interval: Option<TimeInterval>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            gldt_ledger_id: Principal::anonymous(),
            goldao_ledger_id: Principal::anonymous(),
            authorized_principals: vec![],
            whitelist: vec![],
            stake_system: StakeSystem::default(),
            goldao_sns_rewards_canister_id: Principal::anonymous(),
            goldao_sns_governance_canister_id: Principal::anonymous(),
            neuron_system: NeuronSystem::default(),
            reward_claim_interval: Some(TimeInterval {
                weekday: Some("Thursday".to_string()),
                start_hour: 12,
                end_hour: 13,
            }),
            allocate_rewards_interval: Some(TimeInterval {
                weekday: Some("Thursday".to_string()),
                start_hour: 11,
                end_hour: 12,
            }),
            unallocated_rewards_pool: UnallocatedRewardsPool::new_unallocated(),
            processing_rewards_pool: ProcessingRewardsPool::new_processing(),
            allocated_rewards_pool: AllocatedRewardsPool::new_allocated(),
            principal_guards: BTreeSet::new(),
            proposal_system: ProposalSystem::default(),
            analytics_system: AnalyticsSystem::default(),
        }
    }
}
