use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::BuildVersion;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub icp_ledger_canister_id: Principal,
    pub sns_ledger_canister_id: Principal,
    pub ogy_ledger_canister_id: Principal,
    pub sns_gov_canister_id: Principal,
}
