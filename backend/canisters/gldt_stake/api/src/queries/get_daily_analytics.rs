use candid::CandidType;
use gldt_stake_common::daily_analytics::DailyAnalytics;
use serde::{Deserialize, Serialize};
use types::TimestampMillis;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub starting_day: u64,
    pub limit: Option<usize>,
}

pub type Response = Vec<(TimestampMillis, DailyAnalytics)>;
