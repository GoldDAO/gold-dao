use std::time::Duration;

use canister_time::{ run_now_then_interval, WEEK_IN_MS };
use tracing::debug;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(WEEK_IN_MS), run);
}

pub fn run() {
    ic_cdk::spawn(mc_test())
}

pub async fn mc_test() {
    debug!("lets go")
}
