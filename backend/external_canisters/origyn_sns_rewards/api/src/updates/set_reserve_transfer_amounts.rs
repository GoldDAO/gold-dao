use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::ReserveTokenAmounts;
#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Args {
    pub transfer_amounts: ReserveTokenAmounts,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    InternalError(String),
}
