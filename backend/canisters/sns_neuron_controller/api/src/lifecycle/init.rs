use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub sns_rewards_canister_id: Principal,
    pub ogy_sns_governance_canister_id: Principal,
    pub ogy_sns_ledger_canister_id: Principal,
    pub ogy_sns_rewards_canister_id: Principal,
}
