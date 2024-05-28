use candid::CandidType;
use candid::Empty;
use serde::{Deserialize, Serialize};
use types::CanisterId;
use types::Cycles;
use types::TimestampMillis;

pub type Args = Empty;
pub type Response = Vec<CanisterMetrics>;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct CanisterMetrics {
    pub canister_id: CanisterId,
    pub added: TimestampMillis,
    pub top_ups: Vec<CyclesTopUp>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CyclesTopUp {
    pub date: TimestampMillis,
    pub amount: Cycles,
}
