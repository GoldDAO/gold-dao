use std::collections::BTreeSet;

use crate::model::unallocated_rewards_pool::*;
use candid::Principal;
use serde::{Deserialize, Serialize};
use utils::env::CanisterEnv;

use crate::model::allocated_rewards_pool::AllocatedRewardsPool;
use crate::model::processing_rewards_pool::ProcessingRewardsPool;
use crate::{
    model::{neuron_system::NeuronSystem, stake_system::StakeSystem},
    utils::TimeInterval,
};

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    pub env: CanisterEnv,
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub gldt_ledger_id: Principal,
    pub goldao_ledger_id: Principal,
    pub goldao_sns_rewards_canister_id: Principal,
    pub goldao_sns_governance_canister_id: Principal,
    pub authorized_principals: Vec<Principal>,
    pub principal_guards: BTreeSet<Principal>,
    pub stake_system: StakeSystem,
    pub neuron_system: NeuronSystem,
    pub unallocated_rewards_pool: UnallocatedRewardsPool,
    pub processing_rewards_pool: ProcessingRewardsPool,
    pub allocated_rewards_pool: AllocatedRewardsPool,
    pub reward_claim_interval: Option<TimeInterval>,
}
