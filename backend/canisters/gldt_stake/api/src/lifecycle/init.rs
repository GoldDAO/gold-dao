use bity_ic_icrc3::config::ICRC3Config;
use bity_ic_types::BuildVersion;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::TokenSymbol;

#[derive(Deserialize, Serialize, CandidType, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub authorized_principals: Vec<Principal>,
    pub goldao_ledger_id: Principal,
    pub allowed_reward_tokens: Vec<String>,
    pub apy_limit: Option<u8>,
    pub gldt_ledger_id: Principal,
    pub gld_sns_rewards_canister_id: Principal,
    pub gld_sns_governance_canister_id: Principal,
    pub icrc3_config: ICRC3Config,
}
