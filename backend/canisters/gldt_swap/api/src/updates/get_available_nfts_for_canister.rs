use candid::{CandidType, Nat, Principal};
use gldt_swap_common::general_error::GeneralError;
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]

pub struct Args {
    pub principal: Option<Principal>,
    pub canister_id: CanisterId,
}

pub type Response = Result<Vec<Nat>, GeneralError>;
