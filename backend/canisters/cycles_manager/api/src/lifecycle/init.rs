use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::{CanisterId, Cycles};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub authorized_principals: Vec<Principal>,
    pub canisters: Vec<CanisterId>,
    pub sns_root_canister: CanisterId,
    pub max_top_up_amount: Cycles,
    pub min_cycles_balance: Cycles,
}
