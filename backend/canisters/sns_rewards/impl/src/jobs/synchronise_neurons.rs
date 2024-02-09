/*!
# SNS neuron maturity process

This job is responsible for processing the maturity of neurons. It is run every
epoch and processes the maturity of all neurons in this epoch. This maturity
is stored in the canister and is used to determine the rewards that a neuron
is eligible for.
*/

use candid::Principal;
use canister_time::{ now_millis, run_now_then_interval, DAY_IN_MS, HOUR_IN_MS };
use sns_governance_canister::types::{ NeuronId, Neuron };
use tracing::{ debug, error, info, warn };
use std::{ collections::{ btree_map, BTreeMap }, time::Duration };
use types::{ Maturity, Milliseconds, NeuronInfo, TimestampMillis };

use crate::{
    // model::maturity_history::{ insert_event, MaturityEvent },
    model::maturity_history::MaturityHistory,
    state::{ mutate_state, read_state },
};

// set to HOURS for development but modify to DAYS for production
const SYNC_NEURONS_INTERVAL: Milliseconds = HOUR_IN_MS;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(SYNC_NEURONS_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(synchronise_neuron_data())
}

pub async fn synchronise_neuron_data() {
    let canister_id = read_state(|state| state.sns_governance_canister);

    mutate_state(|state| {
        state.sync_info.last_synced_start = now_millis();
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

        debug!("Fetching neuron data");
        match sns_governance_canister_c2c_client::list_neurons(canister_id, &args).await {
            Ok(response) => {
                mutate_state(|state| {
                    let neuron_maturity = &mut state.neuron_maturity;
                    let principal_neurons = &mut state.principal_neurons;
                    let maturity_history = &mut state.maturity_history;
                    debug!("Updating neurons");
                    let mut neuron_infos: Vec<(NeuronId, TimestampMillis, NeuronInfo)> = vec![];
                    response.neurons.iter().for_each(|neuron| {
                        update_principal_neuron_mapping(principal_neurons, neuron);
                        if
                            let Some((neuron_id, neuron_info)) = update_neuron_maturity(
                                neuron_maturity,
                                neuron
                            )
                        {
                            neuron_infos.push((
                                neuron_id,
                                state.sync_info.last_synced_start,
                                neuron_info.into(),
                            ));
                        }
                    });
                    update_neuron_maturity_history(maturity_history, neuron_infos);
                });
                let number_of_received_neurons = response.neurons.len();
                if (number_of_received_neurons as u32) == limit {
                    args.start_page_at = response.neurons.last().map_or_else(
                        || {
                            error!(
                                "Missing last neuron to continue iterating.
                                This should not be possible as the limits are checked. Stopping loop here."
                            );
                            None
                        },
                        |n| {
                            continue_scanning = true;
                            n.id.clone()
                        }
                    );
                }
                number_of_scanned_neurons += number_of_received_neurons;
            }
            Err(err) => {
                let error_message = format!("{err:?}");
                error!(?error_message, "Error fetching neuron data");
            }
        }
    }
    info!("Successfully scanned {number_of_scanned_neurons} neurons.");
    mutate_state(|state| {
        state.sync_info.last_synced_end = now_millis();
        state.sync_info.last_synced_number_of_neurons = number_of_scanned_neurons;
    });
}

// Function to update neuron maturity
fn update_neuron_maturity(
    reg: &mut BTreeMap<NeuronId, NeuronInfo>,
    neuron: &Neuron
) -> Option<(NeuronId, NeuronInfo)> {
    // This function only returns Some() if the neuron is initialised or its maturity has changed
    let mut res: Option<(NeuronId, NeuronInfo)> = None;
    if let Some(id) = &neuron.id {
        let maturity = calculate_total_maturity(neuron);

        let mut neuron_info = NeuronInfo {
            last_synced_maturity: maturity,
            accumulated_maturity: 0,
        };

        // TODO - check age of neuron to avoid someone gaming the system by spawning neurons (check if really relevant)
        match reg.entry(id.clone()) {
            btree_map::Entry::Vacant(entry) => {
                entry.insert(neuron_info);
                res = Some((id.clone(), neuron_info));
            }
            btree_map::Entry::Occupied(mut entry) => {
                let prev_neuron_info = entry.get_mut();
                if let Some(delta) = maturity.checked_sub(prev_neuron_info.last_synced_maturity) {
                    // only add the difference if the maturity has increased
                    neuron_info.accumulated_maturity = prev_neuron_info.accumulated_maturity
                        .checked_add(delta)
                        .unwrap_or(prev_neuron_info.accumulated_maturity);
                    // then overwrite the previous entry
                    prev_neuron_info.accumulated_maturity = neuron_info.accumulated_maturity;
                    prev_neuron_info.last_synced_maturity = neuron_info.last_synced_maturity;
                    // and return the updated neuron info
                    res = Some((id.clone(), neuron_info));
                }
            }
        }
        // for development purposes, we always log the maturity of the neuron to test the history log
        res = Some((id.clone(), neuron_info));
    }
    return res;
}

// Function to insert entry to maturity history of neurons
fn update_neuron_maturity_history(
    maturity_history: &mut MaturityHistory,
    data: Vec<(NeuronId, TimestampMillis, NeuronInfo)>
) {
    maturity_history.insert_multiple(data);
}

// Function to update principal-neuron mapping
fn update_principal_neuron_mapping(prin: &mut BTreeMap<Principal, Vec<NeuronId>>, neuron: &Neuron) {
    // only look at the first permissioned principal, as this is in 99% cases the owner of the neuron
    if let Some(permissioned_principal) = neuron.permissions.first() {
        if let Some(pid) = permissioned_principal.principal {
            prin.entry(pid)
                .and_modify(|neurons| {
                    if let Some(id) = &neuron.id {
                        if !neurons.contains(id) {
                            neurons.push(id.clone());
                        }
                    }
                })
                .or_insert_with(|| {
                    if let Some(id) = &neuron.id { vec![id.clone()] } else { vec![] }
                });
        }
    }
}

fn calculate_total_maturity(neuron: &Neuron) -> Maturity {
    neuron.maturity_e8s_equivalent
        .checked_add(neuron.staked_maturity_e8s_equivalent.unwrap_or(0))
        .unwrap_or_else(|| {
            let id = neuron.id.clone().unwrap_or_default();
            warn!("Unexpected overflow when calculating total maturity of neuron {id}");
            0
        })
}
