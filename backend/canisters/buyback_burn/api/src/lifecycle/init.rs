use candid::{ CandidType, Principal };
use ic_ledger_types::Tokens;
use serde::{ Deserialize, Serialize };
use types::TokenInfo;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub commit_hash: String,
    pub authorized_principals: Vec<Principal>,
    pub gldgov_token_info: TokenInfo,
    pub tokens: Vec<TokenAndPool>,
    pub swap_interval_in_secs: u64,
    pub icp_swap_canister_id: Principal,
    pub burn_rate: u8,
    // NOTE: this value can be used to also define threshold for other tokens by taking the converted price
    pub min_icp_burn_amount: Tokens,
    pub burn_interval_in_secs: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct TokenAndPool {
    pub token: TokenInfo,
    pub swap_pool_id: Principal,
}
