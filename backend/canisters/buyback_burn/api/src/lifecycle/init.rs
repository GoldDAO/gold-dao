use candid::{ CandidType, Principal };
use serde::{ Deserialize, Serialize };
use types::{ CanisterId, TokenInfo };
use ic_ledger_types::Tokens;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub authorized_principals: Vec<Principal>,
    pub gldgov_ledger_canister_id: CanisterId,
    pub tokens: Vec<TokenInfo>,
    pub swap_interval_in_secs: u64,
    pub sns_governance_canister_id: CanisterId,
    pub burn_rate: u8,
    // NOTE: this value can be used to also define threshold for other tokens by taking the converted price
    pub min_icp_burn_amount: Tokens,
    pub burn_interval_in_secs: u64,
}
