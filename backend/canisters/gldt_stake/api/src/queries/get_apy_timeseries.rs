use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::TimestampMillis;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub starting_week: u64,
    pub limit: Option<usize>,
}

pub type Response = Vec<(TimestampMillis, f64)>;
