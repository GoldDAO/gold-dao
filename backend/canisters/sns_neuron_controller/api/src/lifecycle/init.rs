use candid::Nat;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::BuildVersion;
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub authorized_principals: Vec<Principal>,
    pub ogy_manager_config: OgyManagerConfig,
    pub wtn_manager_config: WtnManagerConfig,
    pub sns_rewards_canister_id: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct WtnManagerConfig {
    pub wtn_sns_governance_canister_id: Principal,
    pub wtn_sns_ledger_canister_id: Principal,
    pub icp_ledger: Principal,
    pub icp_rewards_threshold: Nat,
    pub wtn_rewards_threshold: Nat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct OgyManagerConfig {
    pub ogy_sns_governance_canister_id: Principal,
    pub ogy_sns_ledger_canister_id: Principal,
    pub ogy_sns_rewards_canister_id: Principal,
    pub ogy_rewards_threshold: Nat,
}
