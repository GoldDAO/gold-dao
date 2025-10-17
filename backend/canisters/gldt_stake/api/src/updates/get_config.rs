use candid::CandidType;
use serde::{Deserialize, Serialize};

pub type Args = ();

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    pub reward_tokens: Vec<String>,
    pub unlock_delay: u64,
    pub early_unlock_fee: f64,
    pub stake_limit_min: u64,
    pub stake_limit_max: u64,
    pub max_dissolve_events: usize,
}
