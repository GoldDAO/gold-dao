use crate::ICPSwapResult;
use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserUnusedBalance {
    pub balance0: Nat,
    pub balance1: Nat,
}

pub type Args = Principal;
pub type Response = ICPSwapResult<UserUnusedBalance>;
