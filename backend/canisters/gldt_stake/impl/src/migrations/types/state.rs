use crate::model::allocated_rewards_pool::AllocatedRewardsPool;
use crate::model::analytics_system::AnalyticsSystem;
use crate::model::processing_rewards_pool::ProcessingRewardsPool;
use crate::model::proposal_system::ProposalSystem;
use crate::model::stake_system::StakeSystem;
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
    pub whitelist: Vec<Principal>,
    pub principal_guards: BTreeSet<Principal>,
    pub stake_system: StakeSystem,
    pub neuron_system: NeuronSystem,
    pub proposal_system: ProposalSystem,
    pub analytics_system: AnalyticsSystem,
    pub unallocated_rewards_pool: UnallocatedRewardsPool,
    pub processing_rewards_pool: ProcessingRewardsPool,
    pub allocated_rewards_pool: AllocatedRewardsPool,
    pub reward_claim_interval: Option<TimeInterval>,
    pub allocate_rewards_interval: Option<TimeInterval>,
}

// use crate::memory::get_daily_apy_memory;
use candid::Nat;
use gldt_stake_common::stake_position::StakePosition;
use std::collections::HashMap;
use types::TimestampMillis;
use types::TokenSymbol;
