use crate::state::{mutate_state, read_state};
use crate::types::neuron_manager::NeuronManager;
use crate::types::neuron_manager::NeuronRewardsManager;
use canister_time::{run_now_then_interval, DAY_IN_MS};
use canister_tracing_macros::trace;
use std::time::Duration;
use tracing::error;
use types::Milliseconds;

const PROCESS_NEURONS_INTERVAL: Milliseconds = DAY_IN_MS; // 1 day

const CLAIM_REWARDS_THRESHOLD: u64 = 100_000_000 * 1_000_000; // 1_000_000 tokens

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(PROCESS_NEURONS_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

#[trace]
async fn run_async() {
    // NOTE: doublecheck here all the state mutations. Now it seems the most simple way, because
    // if there would be any error with ogy_neuron_manager then it will not affect the state
    let mut ogy_neuron_manager = read_state(|state| state.data.neuron_managers.ogy.clone());
    match ogy_neuron_manager.fetch_and_sync_neurons().await {
        Ok(_) => {
            let available_rewards = ogy_neuron_manager.get_available_rewards().await;
            // TODO: Once the balance exceeds a certain threshold (e.g. 1 million OGY) the rewards should be claimed and distributed
            // Q: Should it be 1 million OGY for all controlled neurons or for each one?
            if available_rewards >= CLAIM_REWARDS_THRESHOLD
                && ogy_neuron_manager.claim_rewards().await.is_not_failed()
            {
                let _ = ogy_neuron_manager.distribute_rewards().await;
            }
            // NOTE: Wrtie all the changes into the state. Here the 'neurons: Neurons' are updated and 'timestamp: TimestampMillis'
            // Q: doesn't it seem stupid to do this in this way?
            mutate_state(|s| s.data.neuron_managers.ogy = ogy_neuron_manager);
        }
        Err(err) => {
            error!("Error fetching and syncing neurons: {:?}", err);
        }
    }
}
