use crate::model::proposal_system::process_proposals;
use canister_time::{run_now_then_interval, HOUR_IN_MS};
use std::time::Duration;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(6 * HOUR_IN_MS), process_proposals_job);
}

pub fn process_proposals_job() {
    ic_cdk::spawn(process_proposals_impl())
}

async fn process_proposals_impl() {
    let _ = process_proposals().await;
}
