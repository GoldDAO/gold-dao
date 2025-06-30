use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

pub type Args = Nat;

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    InternalError(String),
}
