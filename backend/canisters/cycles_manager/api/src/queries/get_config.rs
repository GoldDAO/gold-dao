use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::Cycles;
use types::Milliseconds;

pub type Args = ();
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Response {
    pub max_top_up_amount: Cycles,
    pub min_interval: Milliseconds,
    pub min_cycles_balance: Cycles,
}
