use crate::cache::remove_all_values_older_than;
use bity_ic_canister_time::{run_interval, DAY_IN_MS};
use std::time::Duration;

const CACHE_CLEANUP_INTERVAL: u64 = DAY_IN_MS;

pub fn start_job() {
    run_interval(
        Duration::from_millis(CACHE_CLEANUP_INTERVAL),
        cleanup_cache_job,
    );
}

fn cleanup_cache_job() {
    ic_cdk::futures::spawn(cleanup_cache());
}

async fn cleanup_cache() {
    let timestamp = ic_cdk::api::time();

    remove_all_values_older_than(&timestamp);
}
