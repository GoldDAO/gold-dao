use candid::CandidType;
use candid::Nat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::TokenSymbol;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub starting_day: u64,
    pub limit: Option<usize>,
}

pub type Response = Vec<(u64, HashMap<TokenSymbol, Nat>)>;
