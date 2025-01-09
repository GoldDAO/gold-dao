use candid::{CandidType, Principal};

use gldt_stake_common::reward_tokens::RewardTypes;
use serde::{Deserialize, Serialize};

use types::BuildVersion;

#[derive(Deserialize, Serialize, CandidType, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub authorized_principals: Vec<Principal>,
    pub gldgov_ledger_id: Principal,
    pub gldt_ledger_id: Principal,
    pub gld_sns_rewards_canister_id: Principal,
    pub gld_sns_governance_canister_id: Principal,
    pub reward_types: RewardTypes,
}
