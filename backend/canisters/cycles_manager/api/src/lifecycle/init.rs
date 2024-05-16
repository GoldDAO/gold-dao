use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::{BuildVersion, CanisterId, Cycles, Milliseconds};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub authorized_principals: Vec<Principal>,
    pub canisters: Vec<CanisterId>,
    pub sns_root_canister: Option<CanisterId>,
    pub max_top_up_amount: Cycles,
    pub min_interval: Milliseconds,
    pub min_cycles_balance: Cycles,
    pub wasm_version: BuildVersion,
    pub test_mode: bool,
}
