use candid::{CandidType, Principal};
use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};
use types::BuildVersion;
use types::TokenInfo;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub authorized_principals: Vec<Principal>,
    pub gldgov_token_info: TokenInfo,
    pub tokens: Vec<TokenAndPool>,
    pub buyback_interval_in_secs: u64,
    pub icp_swap_canister_id: Principal,
    pub burn_rate: u8,
    pub min_burn_amount: Tokens, // in GoldGov tokens
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct TokenAndPool {
    pub token: TokenInfo,
    pub swap_pool_id: Principal,
}
