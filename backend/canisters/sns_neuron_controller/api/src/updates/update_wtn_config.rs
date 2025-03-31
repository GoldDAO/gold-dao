use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub wtn_sns_governance_canister_id: Option<Principal>,
    pub wtn_sns_ledger_canister_id: Option<Principal>,
    pub icp_ledger: Option<Principal>,
    pub icp_rewards_threshold: Option<Nat>,
    pub wtn_rewards_threshold: Option<Nat>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    InternalError(String),
}
