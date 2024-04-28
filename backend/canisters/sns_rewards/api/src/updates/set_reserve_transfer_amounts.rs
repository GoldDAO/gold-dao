use std::collections::HashMap;
use candid::{ CandidType, Nat };
use serde::{ Deserialize, Serialize };
use types::TokenSymbol;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub transfer_amounts: HashMap<TokenSymbol, Nat>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Response {
    Success,
    InternalError(String),
}
