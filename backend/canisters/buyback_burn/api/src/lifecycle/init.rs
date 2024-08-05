use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::{CanisterId, Cycles, Milliseconds};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub authorized_principals: Vec<Principal>,
    // pub authorized_principals: HashSet<Principal>,
    pub sns_governance_canister: Option<CanisterId>,
    pub min_burn_amount: Cycles,
}
