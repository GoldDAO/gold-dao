use candid::CandidType;
use serde::{Deserialize, Serialize};

pub type Args = ();

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    InternalError(String),
}
