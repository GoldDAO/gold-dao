use candid::{ CandidType, Nat };
use serde::{ Deserialize, Serialize };

use crate::ReserveTokenAmounts;

pub type Args = ReserveTokenAmounts;

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    InternalError(String),
}
