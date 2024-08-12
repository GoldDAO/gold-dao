use candid::{ CandidType, Principal };
use serde::{ Deserialize, Serialize };
use types::{ CanisterId, TokenInfo, TokenSymbol };
use std::collections::HashMap;
use ic_ledger_types::Tokens;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub authorized_principals: Vec<Principal>,
    // sns_ledger_canister_id
    pub gldgov_ledger_canister_id: CanisterId,
    // Define here min burn amount (it would be perfect to just use ICP equvalent. To do this, we have to see the price +-)
    pub tokens: HashMap<TokenSymbol, TokenInfo>,
    pub sns_governance_canister_id: CanisterId,
    pub burn_rate: u8,
    // 300 ICP
    pub min_icp_burn_amount: Tokens,
    pub burn_interval_in_secs: u64,
}
