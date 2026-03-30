/*!
# SNS neuron maturity process

This job is responsible for processing the maturity of neurons. It is run every
epoch and processes the maturity of all neurons in this epoch. This maturity
is stored in the canister and is used to determine the rewards that a neuron
is eligible for.
*/

use crate::state::{mutate_state, read_state};
use bity_ic_canister_time::{start_job_daily_at, timestamp_millis};
use tracing::{debug, error, info};

pub fn start_job() {
    start_job_daily_at(9, run);
}

pub fn run() {
    ic_cdk::futures::spawn(run_async());
}

async fn run_async() {
    synchronise_neuron_data().await;
}

pub async fn synchronise_neuron_data() {
    ic_cdk::println!("[synchronise_neuron_data] Starting neuron synchronisation");
    if read_state(|s| s.get_is_synchronizing_neurons()) {
        ic_cdk::println!("[synchronise_neuron_data] Already synchronizing neurons, skipping");
        return;
    }
    let canister_id = read_state(|state| state.data.sns_governance_canister);
    let is_test_mode = read_state(|s| s.env.is_test_mode());
    ic_cdk::println!(
        "[synchronise_neuron_data] Canister ID: {:?}, test mode: {}",
        canister_id,
        is_test_mode
    );
    mutate_state(|state| {
        state.data.neuron_system.sync_info.last_synced_start = timestamp_millis();
        state.set_is_synchronizing_neurons(true);
    });

    let mut number_of_scanned_neurons = 0;
    let mut continue_scanning = true;
    // the max limit of 100 is given by the list_neurons call implementation. Cannot increase it.
    let limit = 100;

    let mut args = sns_governance_canister::list_neurons::Args {
        limit,
        start_page_at: None,
        of_principal: None,
    };

    while continue_scanning {
        continue_scanning = false;

        ic_cdk::println!(
            "[synchronise_neuron_data] Fetching neuron page, scanned so far: {}",
            number_of_scanned_neurons
        );
        debug!("Fetching neuron data");
        match sns_governance_canister_c2c_client::list_neurons(canister_id, &args).await {
            Ok(response) => {
                let number_of_received_neurons = response.neurons.len();
                ic_cdk::println!(
                    "[synchronise_neuron_data] Received {} neurons",
                    number_of_received_neurons
                );
                mutate_state(|state| {
                    debug!("Updating neurons");
                    response.neurons.iter().for_each(|neuron| {
                        state.data.neuron_system.upsert_neuron(neuron);
                    });
                });
                if (number_of_received_neurons as u32) == limit {
                    args.start_page_at = response.neurons.last().map_or_else(
                        || {
                            ic_cdk::println!("[synchronise_neuron_data] ERROR: Missing last neuron to continue iterating, stopping loop");
                            error!(
                                "Missing last neuron to continue iterating.
                                This should not be possible as the limits are checked. Stopping loop here."
                            );
                            None
                        },
                        |n| {
                            continue_scanning = true;
                            if is_test_mode && number_of_scanned_neurons >= 400 {
                                ic_cdk::println!("[synchronise_neuron_data] Test mode limit reached at {} neurons, stopping", number_of_scanned_neurons);
                                continue_scanning = false;
                            }
                            n.id.clone()
                        }
                    );
                }
                number_of_scanned_neurons += number_of_received_neurons;
            }
            Err(err) => {
                let error_message = format!("{err:?}");
                ic_cdk::println!(
                    "[synchronise_neuron_data] ERROR fetching neuron data: {}",
                    error_message
                );
                error!(?error_message, "Error fetching neuron data");
            }
        }
    }
    ic_cdk::println!(
        "[synchronise_neuron_data] Completed. Total neurons scanned: {}",
        number_of_scanned_neurons
    );
    info!("Successfully scanned {number_of_scanned_neurons} neurons.");
    mutate_state(|state| {
        state.data.neuron_system.sync_info.last_synced_end = timestamp_millis();
        state
            .data
            .neuron_system
            .sync_info
            .last_synced_number_of_neurons = number_of_scanned_neurons;
        state.set_is_synchronizing_neurons(false);
    });
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use sns_governance_canister::types::{DisburseMaturityInProgress, Neuron, NeuronId};
    use types::NeuronInfo;

    use crate::state::{init_state, mutate_state, read_state, RuntimeState};

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    #[test]
    fn test_insert_update_neuron() {
        init_runtime_state();

        let neuron_id =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();
        let limit = 5;

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());
        neuron.dissolve_state = Some(
            sns_governance_canister::types::neuron::DissolveState::DissolveDelaySeconds(65072000),
        );

        // ********************************
        // 1. Insert new neuron
        // ********************************

        mutate_state(|state| {
            state.data.neuron_system.upsert_neuron(&neuron);
        });

        let mut expected_result = NeuronInfo {
            accumulated_maturity: 0,
            last_synced_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        let mut result = read_state(|state| {
            state
                .data
                .neuron_system
                .neuron_maturity
                .get(&neuron_id)
                .cloned()
        })
        .unwrap();

        assert_eq!(result, expected_result);

        let mut expected_result_history = vec![(0, expected_result)];
        let mut result_history = read_state(|state| {
            state
                .data
                .neuron_system
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);

        // ********************************
        // 2. Increase neuron maturity
        // ********************************

        neuron.maturity_e8s_equivalent = 100;
        neuron.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.neuron_system.sync_info.last_synced_start += 100;
            state.data.neuron_system.upsert_neuron(&neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 150,
            last_synced_maturity: 150,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        result = read_state(|state| {
            state
                .data
                .neuron_system
                .neuron_maturity
                .get(&neuron_id)
                .cloned()
        })
        .unwrap();

        assert_eq!(result, expected_result);

        expected_result_history.push((100, expected_result));
        result_history = read_state(|state| {
            state
                .data
                .neuron_system
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        println!("result_history{:?}", result_history);
        println!("expected_result_history{:?}", expected_result_history);

        assert_eq!(result_history, expected_result_history);

        // ********************************
        // 3. Reduce neuron maturity
        // ********************************

        neuron.maturity_e8s_equivalent = 0;
        neuron.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.neuron_system.sync_info.last_synced_start += 150;
            state.data.neuron_system.upsert_neuron(&neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 150,
            last_synced_maturity: 50,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        result = read_state(|state| {
            state
                .data
                .neuron_system
                .neuron_maturity
                .get(&neuron_id)
                .cloned()
        })
        .unwrap();

        assert_eq!(result, expected_result);

        expected_result_history.push((250, expected_result));
        result_history = read_state(|state| {
            state
                .data
                .neuron_system
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);

        // ********************************
        // 4. No change in neuron maturity
        // ********************************

        neuron.maturity_e8s_equivalent = 0;
        neuron.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.neuron_system.sync_info.last_synced_start += 150;
            state.data.neuron_system.upsert_neuron(&neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 150,
            last_synced_maturity: 50,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        result = read_state(|state| {
            state
                .data
                .neuron_system
                .neuron_maturity
                .get(&neuron_id)
                .cloned()
        })
        .unwrap();

        assert_eq!(result, expected_result);

        // `expected_result_history` stays the same
        result_history = read_state(|state| {
            state
                .data
                .neuron_system
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);
    }

    #[test]
    fn test_neuron_with_disburse_event() {
        init_runtime_state();

        let neuron_id =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();
        let limit = 5;

        let mut neuron = Neuron::default();
        neuron.id = Some(neuron_id.clone());
        neuron.dissolve_state = Some(
            sns_governance_canister::types::neuron::DissolveState::DissolveDelaySeconds(65072000),
        );

        // ********************************
        // 1. Insert new neuron
        // ********************************

        mutate_state(|state| {
            state.data.neuron_system.upsert_neuron(&neuron);
        });

        let mut expected_result = NeuronInfo {
            accumulated_maturity: 0,
            last_synced_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        let mut result = read_state(|state| {
            state
                .data
                .neuron_system
                .neuron_maturity
                .get(&neuron_id)
                .cloned()
        })
        .unwrap();

        assert_eq!(result, expected_result);

        let mut expected_result_history = vec![(0, expected_result)];
        let mut result_history = read_state(|state| {
            state
                .data
                .neuron_system
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);

        // ********************************
        // 2. Increase neuron maturity
        // ********************************

        neuron.maturity_e8s_equivalent = 100;
        neuron.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.neuron_system.sync_info.last_synced_start += 100;
            state.data.neuron_system.upsert_neuron(&neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 150,
            last_synced_maturity: 150,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(0),
        };
        result = read_state(|state| {
            state
                .data
                .neuron_system
                .neuron_maturity
                .get(&neuron_id)
                .cloned()
        })
        .unwrap();

        assert_eq!(result, expected_result);

        expected_result_history.push((100, expected_result));
        result_history = read_state(|state| {
            state
                .data
                .neuron_system
                .maturity_history
                .get_maturity_history(neuron_id.clone(), limit)
        });

        assert_eq!(result_history, expected_result_history);

        // ********************************
        // 3. Reduce neuron maturity with a disburse event
        // ********************************

        neuron.maturity_e8s_equivalent = 0;
        neuron.staked_maturity_e8s_equivalent = None;
        neuron.disburse_maturity_in_progress = vec![DisburseMaturityInProgress {
            amount_e8s: 500u64,
            timestamp_of_disbursement_seconds: 10,
            account_to_disburse_to: None,
        }];

        mutate_state(|state| {
            state.data.neuron_system.sync_info.last_synced_start += 150;
            state.data.neuron_system.upsert_neuron(&neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 500,
            last_synced_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(10),
        };
        result = read_state(|state| {
            state
                .data
                .neuron_system
                .neuron_maturity
                .get(&neuron_id)
                .cloned()
        })
        .unwrap();

        assert_eq!(result, expected_result);

        // ********************************
        // 4. Reduce neuron maturity with a second disburse event but with the other still present as a record
        // ********************************

        neuron.maturity_e8s_equivalent = 0;
        neuron.staked_maturity_e8s_equivalent = None;
        neuron.disburse_maturity_in_progress = vec![
            DisburseMaturityInProgress {
                amount_e8s: 500u64,
                timestamp_of_disbursement_seconds: 10,
                account_to_disburse_to: None,
            },
            DisburseMaturityInProgress {
                amount_e8s: 400u64,
                timestamp_of_disbursement_seconds: 20,
                account_to_disburse_to: None,
            },
        ];

        mutate_state(|state| {
            state.data.neuron_system.upsert_neuron(&neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 900,
            last_synced_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(20),
        };
        result = read_state(|state| {
            state
                .data
                .neuron_system
                .neuron_maturity
                .get(&neuron_id)
                .cloned()
        })
        .unwrap();

        assert_eq!(result, expected_result);

        // ********************************
        // 5. Keep the same disburse events but nothing extra should happen because they were already accounted for
        // ********************************

        neuron.maturity_e8s_equivalent = 0;
        neuron.staked_maturity_e8s_equivalent = None;
        neuron.disburse_maturity_in_progress = vec![
            DisburseMaturityInProgress {
                amount_e8s: 500u64,
                timestamp_of_disbursement_seconds: 10,
                account_to_disburse_to: None,
            },
            DisburseMaturityInProgress {
                amount_e8s: 400u64,
                timestamp_of_disbursement_seconds: 20,
                account_to_disburse_to: None,
            },
        ];

        mutate_state(|state| {
            state.data.neuron_system.sync_info.last_synced_start += 150;
            state.data.neuron_system.upsert_neuron(&neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 900,
            last_synced_maturity: 0,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(20),
        };
        result = read_state(|state| {
            state
                .data
                .neuron_system
                .neuron_maturity
                .get(&neuron_id)
                .cloned()
        })
        .unwrap();

        assert_eq!(result, expected_result);

        // ********************************
        // 6. increase the normal maturity
        // ********************************

        neuron.maturity_e8s_equivalent = 50;
        neuron.staked_maturity_e8s_equivalent = None;
        neuron.disburse_maturity_in_progress = vec![
            DisburseMaturityInProgress {
                amount_e8s: 500u64,
                timestamp_of_disbursement_seconds: 10,
                account_to_disburse_to: None,
            },
            DisburseMaturityInProgress {
                amount_e8s: 400u64,
                timestamp_of_disbursement_seconds: 20,
                account_to_disburse_to: None,
            },
        ];

        mutate_state(|state| {
            state.data.neuron_system.sync_info.last_synced_start += 150;
            state.data.neuron_system.upsert_neuron(&neuron);
        });

        expected_result = NeuronInfo {
            accumulated_maturity: 950,
            last_synced_maturity: 50,
            rewarded_maturity: HashMap::new(),
            last_disburse_event_considered: Some(20),
        };
        result = read_state(|state| {
            state
                .data
                .neuron_system
                .neuron_maturity
                .get(&neuron_id)
                .cloned()
        })
        .unwrap();

        assert_eq!(result, expected_result);
    }
}
