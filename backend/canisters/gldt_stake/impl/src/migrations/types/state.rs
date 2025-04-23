use std::collections::BTreeSet;

use candid::Principal;
use serde::{Deserialize, Serialize};
use utils::env::CanisterEnv;

use crate::{
    model::{
        archive_system::ArchiveSystem, neuron_system::NeuronSystem, reward_system::RewardSystem,
        stake_system::StakeSystem,
    },
    utils::TimeInterval,
};

#[derive(Serialize, Deserialize)]
pub struct RuntimeStateV0 {
    /// Runtime environment
    pub env: CanisterEnv,
    /// Runtime data
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub gldt_ledger_id: Principal,
    pub ogy_ledger_id: Principal,
    pub goldao_ledger_id: Principal,
    pub icp_ledger_id: Principal,
    pub goldao_sns_rewards_canister_id: Principal,
    pub goldao_sns_governance_canister_id: Principal,
    pub authorized_principals: Vec<Principal>,
    pub principal_guards: BTreeSet<Principal>,
    pub stake_system: StakeSystem,
    pub neuron_system: NeuronSystem,
    pub reward_system: RewardSystem,
    pub archive_system: ArchiveSystem,
    pub reward_claim_interval: Option<TimeInterval>,
    pub is_reward_claim_in_progress: bool,
    pub is_reward_allocation_in_progress: bool,
    pub is_archive_cron_running: bool,
}
