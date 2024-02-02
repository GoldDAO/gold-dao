/*!
# SNS neuron maturity process

This job is responsible for processing the maturity of neurons. It is run every
epoch and processes the maturity of all neurons in this epoch. This maturity
is stored in the canister and is used to determine the rewards that a neuron
is eligible for.
*/

use candid::Principal;

use sns_governance_canister::types::{ NeuronId, Neuron };

use std::collections::{ btree_map, BTreeMap };

use types::{ Maturity, TimestampSeconds };

use crate::state::{ mutate_state, NeuronInfo };

pub type MaturityHistory = Vec<(TimestampSeconds, Maturity)>;

pub async fn update_neuron_data() -> Result<u64, String> {
    // let canister_id = Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").unwrap();
    let canister_id = Principal::from_text("2jvtu-yqaaa-aaaaq-aaama-cai").unwrap();

    let mut number_of_scanned_neurons: u64 = 0;
    let mut continue_scanning = true;
    let limit = 100;

    let mut args = sns_governance_canister::list_neurons::Args {
        limit,
        start_page_at: None,
        of_principal: None,
    };

    while continue_scanning {
        continue_scanning = false;

        ic_cdk::println!("Fetching neuron data");
        match sns_governance_canister_c2c_client::list_neurons(canister_id, &args).await {
            Ok(response) => {
                mutate_state(|state| {
                    let neuron_maturity = &mut state.neuron_maturity;
                    let principal_neurons = &mut state.principal_neurons;
                    response.neurons.iter().for_each(|neuron| {
                        update_neuron_maturity(neuron_maturity, neuron);
                        update_principal_neuron_mapping(principal_neurons, neuron)
                    });
                });
                let number_of_received_neurons = response.neurons.len() as u64;
                if (number_of_received_neurons as u32) == limit {
                    continue_scanning = true;
                    args.start_page_at = response.neurons.last().map_or_else(
                        || {
                            ic_cdk::api::trap(
                                "Missing last neuron to continue iterating. This should not be possible as the limits are checked."
                            )
                        },
                        |n| n.id.clone()
                    );
                }
                number_of_scanned_neurons += number_of_received_neurons;
            }
            Err(err) => {
                ic_cdk::println!("err: {:?}", err);
                // add proper proper logging and tracing here
            }
        }
        // for testing
        if number_of_scanned_neurons >= 300 {
            break;
        }
    }
    Ok(number_of_scanned_neurons)
}

// Function to update neuron maturity
fn update_neuron_maturity(reg: &mut BTreeMap<NeuronId, NeuronInfo>, neuron: &Neuron) {
    if let Some(id) = &neuron.id {
        let maturity =
            neuron.maturity_e8s_equivalent + neuron.staked_maturity_e8s_equivalent.unwrap_or(0);
        match reg.entry(id.clone()) {
            btree_map::Entry::Vacant(entry) => {
                entry.insert(NeuronInfo {
                    last_synced_maturity: maturity,
                    accumulated_maturity: 0,
                });
            }
            btree_map::Entry::Occupied(mut entry) => {
                let info = entry.get_mut();
                let delta = maturity - info.last_synced_maturity;
                if delta != 0 {
                    // only add the difference if the maturity has increased
                    info.accumulated_maturity += std::cmp::max(delta, 0);
                    info.last_synced_maturity = maturity;
                }
            }
        }
    }
}

// // Function to update history
// fn update_neuron_history(hist: &mut BTreeMap<NeuronId, MaturityHistory>, neuron: &Neuron) {
//     if let Some(id) = &neuron.id {
//         let maturity = neuron.maturity_e8s_equivalent;
//         match hist.entry(id.clone()) {
//             btree_map::Entry::Vacant(entry) => {
//                 entry.insert(vec![(timestamp_seconds(), maturity)]);
//             }
//             btree_map::Entry::Occupied(mut entry) => {
//                 entry.get_mut().push((timestamp_seconds(), maturity));
//             }
//         }
//     }
// }

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
