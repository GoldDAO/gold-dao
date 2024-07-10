use candid::{ CandidType, Principal };
use serde::Deserialize;

#[derive(Deserialize, CandidType)]
pub struct InitArgs {
    pub test_mode: bool,
    pub sns_ledger_canister_id: Principal,
    pub sns_gov_canister_id: Principal,
}
