use crate::state::{mutate_state, read_state};
use crate::types::neuron_manager::NeuronManager;
use crate::types::neuron_manager::NeuronRewardsManager;
use crate::types::OgyManager;
use canister_time::{run_now_then_interval, MINUTE_IN_MS};
use canister_tracing_macros::trace;
use std::time::Duration;
use tracing::error;
use tracing::info;
use types::Milliseconds;
use utils::env::Environment;

const PROCESS_NEURONS_INTERVAL: Milliseconds = MINUTE_IN_MS; // 1 day
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
    ic_cdk::println!("Starting neuron processing loop");

    if let Err(err) = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, || async {
        let mut ogy_neuron_manager = read_state(|state| state.data.neuron_managers.ogy.clone());
        fetch_and_process_neurons(&mut ogy_neuron_manager).await
    })
    .await
    {
        error!(
            "Failed to process neurons after {} attempts: {:?}",
            MAX_ATTEMPTS, err
        );
        ic_cdk::println!("Failed to process neurons after");
        crate::jobs::process_neurons::run();
    }
}

async fn fetch_and_process_neurons(ogy_neuron_manager: &mut OgyManager) -> Result<(), String> {
    ogy_neuron_manager
        .fetch_and_sync_neurons()
        .await
        .map_err(|err| {
            error!("Error fetching and syncing neurons: {:?}", err);
            ic_cdk::println!("Error fetching and syncing neurons");
            err.to_string()
        })?;

    let available_rewards = ogy_neuron_manager.get_available_rewards().await;
    if available_rewards >= CLAIM_REWARDS_THRESHOLD
        && ogy_neuron_manager.claim_rewards().await.is_not_failed()
    {
        let _ = ogy_neuron_manager.distribute_rewards().await;
    }

    mutate_state(|s| {
        s.data.neuron_managers.ogy = ogy_neuron_manager.clone();
        s.data.neuron_managers.now = s.env.now();
    });

    Ok(())
}

// TODO: think on how to add delay here
async fn retry_with_attempts<F, Fut>(
    max_attempts: u8,
    delay_duration: Duration,
    mut f: F,
) -> Result<(), String>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<(), String>>,
{
    for attempt in 1..=max_attempts {
        match f().await {
            Ok(_) => return Ok(()),
            Err(err) => {
                error!("Attempt {}: Error - {:?}", attempt, err);
                if attempt == max_attempts {
                    return Err(err);
                }
            }
        }
    }
    Ok(())
}
