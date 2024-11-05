use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::TokenInfo;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub token_list: Vec<(String, TokenInfo)>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    InternalError(String),
}
