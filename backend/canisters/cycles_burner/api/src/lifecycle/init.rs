use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::{BuildVersion, CanisterId, Cycles, Milliseconds};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    /// Interval between timers in seconds.
    pub interval_between_timers_in_seconds: u128,
    /// Amount of burned cycles per timer.
    pub burn_amount: u128,
}
