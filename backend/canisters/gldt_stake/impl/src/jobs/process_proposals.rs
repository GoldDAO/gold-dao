use crate::model::proposal_system::process_proposals;
use bity_ic_canister_time::{run_now_then_interval, HOUR_IN_MS};
use std::time::Duration;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(6 * HOUR_IN_MS), process_proposals_job);
}

pub fn process_proposals_job() {
    ic_cdk::futures::spawn(process_proposals_impl())
}

async fn process_proposals_impl() {
    let _span = tracing::info_span!("PROCESS_PROPOSALS").entered();
    let _ = process_proposals().await;
}
