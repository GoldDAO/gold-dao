use crate::model::allocated_rewards_pool::AllocatedRewardsPool;
use crate::model::processing_rewards_pool::ProcessingRewardsPool;
use crate::model::proposal_system::ProposalSystem;
use crate::model::unallocated_rewards_pool::*;
use crate::{model::neuron_system::NeuronSystem, utils::TimeInterval};

use candid::Principal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use utils::env::CanisterEnv;

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    pub env: CanisterEnv,
    pub data: DataV0,
}

#[derive(Serialize, Deserialize)]
pub struct DataV0 {
    pub gldt_ledger_id: Principal,
    pub goldao_ledger_id: Principal,
    pub goldao_sns_rewards_canister_id: Principal,
    pub goldao_sns_governance_canister_id: Principal,
    pub authorized_principals: Vec<Principal>,
    pub principal_guards: BTreeSet<Principal>,
    pub stake_system: StakeSystemV0,
    pub neuron_system: NeuronSystem,
    pub proposal_system: ProposalSystem,
    pub analytics_system: AnalyticsSystemV0,
    pub unallocated_rewards_pool: UnallocatedRewardsPool,
    pub processing_rewards_pool: ProcessingRewardsPool,
    pub allocated_rewards_pool: AllocatedRewardsPool,
    pub reward_claim_interval: Option<TimeInterval>,
    pub allocate_rewards_interval: Option<TimeInterval>,
}

use candid::Nat;
use gldt_stake_common::stake_position::StakePosition;
use std::collections::HashMap;
use types::TimestampMillis;
use types::TokenSymbol;
#[derive(Serialize, Deserialize)]
pub struct StakeSystemV0 {
    pub stakes: HashMap<Principal, StakePosition>,
    pub total_staked: Nat,
    pub cached_total_weighted_stake: Nat,
    pub stake_positions_quantity_limit: usize,
    pub reward_types: BTreeSet<TokenSymbol>,
    pub pending_fee_transfer_amount: Nat,
    pub genesis_datetime: TimestampMillis,
    pub token_usd_values: HashMap<TokenSymbol, f64>,
    pub cached_daily_timestamp: TimestampMillis,
}

use crate::memory::VM;
use crate::model::analytics_system::init_daily_analytics_history;
use gldt_stake_common::daily_analytics::DailyAnalytics;
use ic_stable_structures::StableBTreeMap;
#[derive(Serialize, Deserialize)]
pub struct AnalyticsSystemV0 {
    pub last_updated_timestamp: TimestampMillis,
    #[serde(skip, default = "init_daily_analytics_history")]
    pub daily_analytics: StableBTreeMap<TimestampMillis, DailyAnalytics, VM>,
}
