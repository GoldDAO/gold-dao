use crate::types::CandidEvent;
use candid::CandidType;
use serde::Deserialize;

#[derive(candid::CandidType, Deserialize)]
pub struct GetEventsArg {
    pub start: u64,
    pub length: u64,
}

#[derive(CandidType, Debug, Clone)]
pub struct GetEventsResult {
    pub events: Vec<CandidEvent>,
    pub total_event_count: u64,
}

pub type Args = GetEventsArg;
pub type Response = GetEventsResult;
