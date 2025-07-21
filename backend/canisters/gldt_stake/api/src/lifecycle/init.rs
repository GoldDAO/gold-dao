use bity_ic_icrc3::config::ICRC3Config;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::{BuildVersion, TokenSymbol};

#[derive(Deserialize, Serialize, CandidType, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub authorized_principals: Vec<Principal>,
    // NOTE: added for testing purposes
    pub whitelist: Vec<Principal>,
    pub goldao_ledger_id: Principal,
    pub allowed_reward_tokens: Vec<TokenSymbol>,
    pub gldt_ledger_id: Principal,
    pub gld_sns_rewards_canister_id: Principal,
    pub gld_sns_governance_canister_id: Principal,
    pub icrc3_config: ICRC3Config,
}
