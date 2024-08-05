use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::CanisterId;
use types::Cycles;
use types::TimestampMillis;

pub type Args = ();
pub type Response = Vec<CanisterTopUp>;

#[derive(CandidType, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct CanisterTopUp {
    pub timestamp: TimestampMillis,
    pub canister_id: CanisterId,
    pub amount: Cycles,
}
