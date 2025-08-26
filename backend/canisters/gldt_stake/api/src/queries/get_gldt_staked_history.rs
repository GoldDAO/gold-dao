use candid::CandidType;
use candid::Nat;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use types::TimestampMillis;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub starting_day: u64,
    pub limit: Option<usize>,
}
pub type Response = BTreeMap<TimestampMillis, Nat>;
