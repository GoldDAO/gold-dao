use crate::state::{mutate_state, read_state};
use crate::types::neuron_manager::NeuronManager;
use crate::types::neuron_manager::NeuronRewardsManager;
use crate::utils::distribute_rewards;
use crate::utils::retry_with_attempts;
use canister_time::{run_now_then_interval, DAY_IN_MS};
use canister_tracing_macros::trace;
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;
use tracing::{error, info};
use types::Milliseconds;
use utils::env::Environment;

const PROCESS_NEURONS_INTERVAL: Milliseconds = DAY_IN_MS; // 1 day
const MAX_ATTEMPTS: u8 = 3;
const RETRY_DELAY: Duration = Duration::from_secs(5 * 60); // each 5 minutes

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(PROCESS_NEURONS_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

#[trace]
async fn run_async() {
    if let Err(err) = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, || async {
        let mut wtn_neuron_manager = read_state(|state| state.data.neuron_managers.wtn.clone());
        fetch_and_process_wtn_neurons(&mut wtn_neuron_manager).await
    })
    .await
    {
        error!(
            "Failed to process neurons after {} attempts: {:?}",
            MAX_ATTEMPTS, err
        );
    }
}

use crate::types::WtnManager;
async fn fetch_and_process_wtn_neurons(wtn_neuron_manager: &mut WtnManager) -> Result<(), String> {
    wtn_neuron_manager
        .fetch_and_sync_neurons()
        .await
        .map_err(|err| {
            error!("Error fetching and syncing neurons: {:?}", err);
            err.to_string()
        })?;

    let available_rewards = wtn_neuron_manager.get_available_rewards().await;
    let wtn_rewards_threshold =
        read_state(|state| state.data.neuron_managers.wtn.wtn_rewards_threshold.clone());

    if available_rewards > wtn_rewards_threshold
        && wtn_neuron_manager.claim_rewards().await.is_not_failed()
    {
        info!("Claimed rewards");
    }

    let icp_rewards_threshold =
        read_state(|state| state.data.neuron_managers.wtn.icp_rewards_threshold.clone());
    let available_icp_rewards = icrc_ledger_canister_c2c_client::icrc1_balance_of(
        wtn_neuron_manager.icp_ledger,
        &Account {
            owner: ic_cdk::id(),
            subaccount: None,
        },
    )
    .await
    .map_err(|e| {
        let msg = format!(
            "Failed to fetch token balance of ledger canister id {}: {:?}",
            wtn_neuron_manager.icp_ledger, e
        );
        error!("{}", msg);
        msg
    })?;

    if available_icp_rewards >= icp_rewards_threshold {
        let _ = distribute_rewards(wtn_neuron_manager.icp_ledger).await;
    }

    mutate_state(|s| {
        s.data.neuron_managers.wtn = wtn_neuron_manager.clone();
        s.data.neuron_managers.now = s.env.now();
    });

    Ok(())
}
