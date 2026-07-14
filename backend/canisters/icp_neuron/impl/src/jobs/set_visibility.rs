use crate::state::read_state;
use crate::updates::manage_nns_neuron::manage_nns_neuron_impl;
use futures::future::join_all;
use nns_governance_canister::types::{
    manage_neuron::{configure::Operation, Command, Configure},
    ListNeurons, SetVisibility,
};
use tracing::{error, info, warn};

pub fn start_job() {
    ic_cdk_timers::set_timer(std::time::Duration::from_secs(1), || {
        ic_cdk::futures::spawn(run_async());
    });
}

pub async fn run_async() {
    info!("Starting one-time neuron visibility setting job...");

    let nns_governance_canister_id = read_state(|state| state.data.nns_governance_canister_id);

    match nns_governance_canister_c2c_client::list_neurons(
        nns_governance_canister_id,
        &(ListNeurons {
            neuron_ids: Vec::new(),
            include_neurons_readable_by_caller: true,
        }),
    )
    .await
    {
        Ok(response) => {
            // Find all neurons that don't have public visibility set (2 is Public)
            let neurons_to_update: Vec<u64> = response
                .full_neurons
                .iter()
                .filter(|n| n.visibility != Some(2))
                .filter_map(|n| n.id.as_ref().map(|id| id.id))
                .collect();

            if neurons_to_update.is_empty() {
                info!("All listed neurons already have public visibility set.");
                return;
            }

            info!(
                count = neurons_to_update.len(),
                "Setting public visibility for neurons concurrently"
            );

            // 1. Keep the ID and its Future tightly bound in a single tuple vector
            let mut pending_updates = Vec::with_capacity(neurons_to_update.len());

            for neuron_id in neurons_to_update {
                let command = Command::Configure(Configure {
                    operation: Some(Operation::SetVisibility(SetVisibility {
                        visibility: Some(2), // 2 = Public
                    })),
                });

                let future = manage_nns_neuron_impl(neuron_id, command);
                pending_updates.push((neuron_id, future));
            }

            let (neuron_ids, futures): (Vec<u64>, Vec<_>) = pending_updates.into_iter().unzip();

            let results = join_all(futures).await;

            let mut all_success = true;
            for (neuron_id, result) in neuron_ids.into_iter().zip(results) {
                match result {
                    Ok(_) => {
                        info!(neuron_id, "Successfully set neuron visibility to public.");
                    }
                    Err(err) => {
                        error!(
                            neuron_id,
                            "Failed to set neuron visibility to public: {:?}", err
                        );
                        all_success = false;
                    }
                }
            }

            if all_success {
                info!("Completed neuron visibility job successfully.");
            } else {
                warn!(
                    "Some neurons failed to update visibility. Will retry on next restart/upgrade."
                );
            }
        }
        Err(err) => {
            error!("Failed to fetch neurons list: {:?}", err);
        }
    }
}
