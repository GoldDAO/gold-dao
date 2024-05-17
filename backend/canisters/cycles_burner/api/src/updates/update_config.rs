use candid::CandidType;
use human_readable::HumanReadable;
// use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};
use types::{Cycles, Milliseconds};

#[derive(CandidType, Serialize, Deserialize, HumanReadable, Clone, Debug, Default)]
pub struct Args {
    /// Interval between timers in seconds.
    pub interval_between_timers_in_seconds: u128,
    /// Amount of burned cycles per timer.
    pub burn_amount: u128,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
}
