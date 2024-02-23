use crate::updates::manage_nns_neuron::manage_nns_neuron_impl;
use crate::state::{ mutate_state, read_state, Neurons };
use canister_time::{ run_now_then_interval, DAY_IN_MS, MINUTE_IN_MS };
use icrc_ledger_types::icrc1::account::Account;
use ledger_utils::icrc_account_to_legacy_account_id;
use nns_governance_canister::types::{
    manage_neuron::{ disburse::Amount, Command, Disburse, Spawn },
    Neuron,
};
use nns_governance_canister::types::ListNeurons;
use utils::{ consts::E8S_PER_ICP, env::Environment };
use std::time::Duration;
use tracing::{ error, info, trace, warn };
use types::Milliseconds;

// We add a minute because spawning takes 7 days, and if we wait exactly 7 days, there may still be a few seconds left
// before the neuron can be spawned
const REFRESH_NEURONS_INTERVAL: Milliseconds = DAY_IN_MS + MINUTE_IN_MS;

const SPAWN_LIMIT_ICP: u64 = 1000;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(REFRESH_NEURONS_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

async fn run_async() {
    let nns_governance_canister_id = read_state(|state| state.data.nns_governance_canister_id);

    match
        nns_governance_canister_c2c_client::list_neurons(
            nns_governance_canister_id,
            &(ListNeurons {
                neuron_ids: Vec::new(),
                include_neurons_readable_by_caller: true,
            })
        ).await
    {
        Ok(response) => {
            let now = read_state(|state| state.env.now());

            let neurons_to_spawn: Vec<_> = response.full_neurons
                .iter()
                .filter(
                    |n|
                        n.spawn_at_timestamp_seconds.is_none() &&
                        n.maturity_e8s_equivalent > SPAWN_LIMIT_ICP * E8S_PER_ICP
                )
                .filter_map(|n| n.id.as_ref().map(|id| id.id))
                .collect();

            let neurons_to_disburse: Vec<Neuron> = response.full_neurons
                .iter()
                .filter(|n| n.is_dissolved(now) && n.cached_neuron_stake_e8s > 0)
                .cloned()
                .collect();

            mutate_state(|state| {
                let mut active_neurons = Vec::new();
                let mut spawning_neurons = Vec::new();
                let mut disbursed_neurons = Vec::new();
                for neuron in response.full_neurons.into_iter() {
                    if neuron.maturity_e8s_equivalent == 0 && neuron.cached_neuron_stake_e8s == 0 {
                        if let Some(neuron_id) = neuron.id {
                            disbursed_neurons.push(neuron_id.id);
                        }
                    } else if neuron.spawn_at_timestamp_seconds.is_some() {
                        spawning_neurons.push(neuron);
                    } else {
                        active_neurons.push(neuron);
                    }
                }

                state.data.neurons = Neurons {
                    timestamp: now,
                    active_neurons,
                    spawning_neurons,
                    disbursed_neurons,
                };
            });

            let mut neurons_updated = false;
            if !neurons_to_spawn.is_empty() {
                spawn_neurons(neurons_to_spawn).await;
                neurons_updated = true;
            }

            if !neurons_to_disburse.is_empty() {
                disburse_neurons(neurons_to_disburse).await;
                neurons_updated = true;
            }

            if neurons_updated {
                // Refresh the neurons again given that they've been updated
                ic_cdk_timers::set_timer(Duration::ZERO, || ic_cdk::spawn(run_async()));
            }
        }
        Err(err) => { error!("Error fetching neuron list: {err:?}") }
    }
}

async fn spawn_neurons(neuron_ids: Vec<u64>) {
    for neuron_id in neuron_ids {
        info!(neuron_id, "Spawning neuron from maturity");
        match manage_nns_neuron_impl(neuron_id, Command::Spawn(Spawn::default())).await {
            Ok(_) => info!("Successfully spawned neuron {neuron_id}."),
            Err(err) => warn!("Error spawning neuron {neuron_id}: {err}"),
        }
    }
}

async fn disburse_neurons(neurons: Vec<Neuron>) {
    let rewards_recipients = read_state(|state| state.data.rewards_recipients.clone());

    for neuron in neurons {
        let neuron_id: u64;
        if let Some(id) = neuron.id {
            neuron_id = id.id;
            info!(id.id, "Disbursing neuron.");
        } else {
            warn!("Empty neuron id. Cannot disburse and continuing to next one.");
            continue;
        }

        let total_amount = neuron.cached_neuron_stake_e8s;

        let payments_list: Vec<(Account, u64)>;
        match rewards_recipients.split_amount_to_each_recipient(total_amount) {
            Ok(list) => {
                payments_list = list;
            }
            Err(err) => {
                warn!(
                    "Error splitting amount to each recipient for neuron {neuron_id}. Error: {err}"
                );
                continue;
            }
        }

        for (icrc_account, amount) in payments_list {
            let icp_ledger_account = nns_governance_canister::types::AccountIdentifier {
                hash: icrc_account_to_legacy_account_id(icrc_account).as_ref().to_vec(),
            };
            match
                manage_nns_neuron_impl(
                    neuron_id,
                    Command::Disburse(Disburse {
                        to_account: Some(icp_ledger_account),
                        amount: Some(Amount { e8s: amount }),
                    })
                ).await
            {
                Ok(_) => (),
                Err(_) => (),
            }
        }
    }
}
