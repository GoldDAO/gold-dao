use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use sns_root_canister::get_sns_canisters_summary::CanisterSummary;
use std::time::Duration;
use tracing::error;
use types::{CanisterId, Cycles, Empty};
use utils::canister::deposit_cycles;

const INTERVAL: Duration = Duration::from_secs(12 * 60 * 60); // 12 hours

use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Canister configuration.
#[derive(Clone, Debug, CandidType, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Interval between timers in seconds.
    pub interval_between_timers_in_seconds: u64,

    /// Amount of burned cycles per timer.
    pub burn_amount: u128,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            // Default interval between timers in one day.
            interval_between_timers_in_seconds: 12 * 60 * 60, // 1 day
            burn_amount: 200_000_000_000_000,
        }
    }
}

pub fn start_job() {
    let config = Config::default();
    run_now_then_interval(
        Duration::from_secs(config.interval_between_timers_in_seconds),
        run,
    );
}

fn run() {
    let config = Config::default();
    ic_cdk::spawn(run_async(config));
}

#[trace]
async fn run_async(config: Config) {
    ic_cdk::api::cycles_burn(config.burn_amount);
}
