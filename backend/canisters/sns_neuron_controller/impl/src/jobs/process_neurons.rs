use crate::state::{ mutate_state, read_state };
use crate::types::neuron_manager::NeuronManager;
use crate::types::neuron_manager::NeuronRewardsManager;
use crate::types::OgyManager;
use canister_time::{ run_now_then_interval, DAY_IN_MS };
use canister_tracing_macros::trace;
use std::time::Duration;
use tracing::error;
use types::Milliseconds;
use utils::env::Environment;
use utils::retry_async::retry_with_attempts;
use anyhow::{ Result, anyhow };

const PROCESS_NEURONS_INTERVAL: Milliseconds = DAY_IN_MS; // 1 day
const MAX_ATTEMPTS: u8 = 3;
const CLAIM_REWARDS_THRESHOLD: u64 = 100_000_000 * 1_000_000; // 1_000_000 tokens
const RETRY_DELAY: Duration = Duration::from_secs(5 * 60); // each 5 minutes

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(PROCESS_NEURONS_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

#[trace]
async fn run_async() {
    if
        let Err(err) = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, || async {
            let mut ogy_neuron_manager = read_state(|state| state.data.neuron_managers.ogy.clone());
            fetch_and_process_neurons(&mut ogy_neuron_manager).await
        }).await
    {
        error!("Failed to process neurons after {} attempts: {:?}", MAX_ATTEMPTS, err);
    }
}

async fn fetch_and_process_neurons(ogy_neuron_manager: &mut OgyManager) -> Result<()> {
    ogy_neuron_manager.fetch_and_sync_neurons().await.map_err(|err| {
        error!("Error fetching and syncing neurons: {:?}", err);
        anyhow!(err)
    })?;

    let available_rewards = ogy_neuron_manager.get_available_rewards().await;
    if
        available_rewards >= CLAIM_REWARDS_THRESHOLD &&
        ogy_neuron_manager.claim_rewards().await.is_not_failed()
    {
        let _ = ogy_neuron_manager.distribute_rewards().await;
    }

    mutate_state(|s| {
        s.data.neuron_managers.ogy = ogy_neuron_manager.clone();
        s.data.neuron_managers.now = s.env.now();
    });

    Ok(())
}
