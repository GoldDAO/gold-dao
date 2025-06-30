use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{TokenInfo, TokenSymbol};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub token_list: Vec<(TokenSymbol, TokenInfo)>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    InternalError(String),
}
