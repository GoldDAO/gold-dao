use crate::model::processing_rewards_pool::ProcessingRewardsPool;
use crate::model::proposal_system::ProposalSystem;
use crate::model::unallocated_rewards_pool::*;
use crate::{model::neuron_system::NeuronSystem, utils::TimeInterval};
use candid::CandidType;
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
    pub stake_system: StakeSystemV0,
    pub neuron_system: NeuronSystem,
    pub proposal_system: ProposalSystem,
    pub unallocated_rewards_pool: UnallocatedRewardsPool,
    pub processing_rewards_pool: ProcessingRewardsPool,
    pub allocated_rewards_pool: AllocatedRewardsPoolV0,
    pub reward_claim_interval: Option<TimeInterval>,
    pub allocate_rewards_interval: Option<TimeInterval>,
}

use crate::memory::get_daily_apy_memory;
use crate::memory::VM;
use candid::Nat;
use gldt_stake_common::stake_position::StakePosition;
use ic_stable_structures::StableBTreeMap;
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
    #[serde(skip, default = "init_daily_apy_history")]
    pub daily_apy_history: StableBTreeMap<TimestampMillis, f64, VM>,
    pub daily_apy_timestamp: TimestampMillis,
    pub daily_weighted_staked_gldt: HashMap<TimestampMillis, Nat>,
}

fn init_daily_apy_history() -> StableBTreeMap<TimestampMillis, f64, VM> {
    let memory = get_daily_apy_memory();
    StableBTreeMap::init(memory)
}

use crate::model::allocated_rewards_pool::AllocatedRewardsState;
use std::collections::BTreeMap;
#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct AllocatedRewardsPoolV0 {
    pub state: AllocatedRewardsState,
    pub last_allocation_time: TimestampMillis,
    pub reward_history: HashMap<TokenSymbol, Nat>, // all the previous rewards added together when a transfer from processing pool has been processed. useful for APY calculations
    pub daily_allocated_rewards: BTreeMap<TimestampMillis, HashMap<TokenSymbol, Nat>>, // daily reward history - keeps track of the total rewards for each week that have been allocated for each token
}
