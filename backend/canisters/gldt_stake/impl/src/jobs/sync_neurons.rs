use crate::model::neuron_system::sync_neurons;
use canister_time::{run_now_then_interval, HOUR_IN_MS};
use std::time::Duration;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(HOUR_IN_MS), sync_neurons_job);
}

pub fn sync_neurons_job() {
    ic_cdk::spawn(sync_neurons_impl())
}

async fn sync_neurons_impl() {
    let _ = sync_neurons().await;
}
