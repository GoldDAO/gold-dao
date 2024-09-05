use candid::CandidType;
use crate::init::TokenAndPool;
use serde::{ Deserialize, Serialize };

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Args {
    pub tokens: Vec<TokenAndPool>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
}
