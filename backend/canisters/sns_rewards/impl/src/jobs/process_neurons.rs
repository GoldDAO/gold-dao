/*!
# SNS neuron maturity process

This job is responsible for processing the maturity of neurons. It is run every
epoch and processes the maturity of all neurons in this epoch. This maturity
is stored in the canister and is used to determine the rewards that a neuron
is eligible for.
*/

use candid::{ CandidType, Principal };
use ic_cdk::query;
use sns_governance_canister::types::{ NeuronId, Neuron };

use std::cell::RefCell;
use std::collections::{ btree_map, BTreeMap };

use types::{ Maturity, TimestampSeconds };
use canister_time::timestamp_seconds;

#[derive(CandidType, Clone)]
pub struct NeuronInfo {
    last_maturity: u64,
    accumulated_maturity: u64,
}

pub type MaturityHistory = Vec<(TimestampSeconds, Maturity)>;

thread_local! {
    static NEURON_REGISTRY: RefCell<BTreeMap<NeuronId, NeuronInfo>> = RefCell::default();
    static PRINCIPAL_NEURONS: RefCell<BTreeMap<Principal, Vec<NeuronId>>> = RefCell::default();
    static MATURITY_HISTORY: RefCell<BTreeMap<NeuronId, MaturityHistory>> = RefCell::default();
}

pub async fn fetch_neuron_data() -> Result<u64, String> {
    // 1. fetch maturity of all neurons
    //    a. call `list_neurons` from SNS governance canister
    //    b. update the internal structure of maturity storage

    // let canister_id = Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").unwrap();
    let canister_id = Principal::from_text("2jvtu-yqaaa-aaaaq-aaama-cai").unwrap();
    let args = sns_governance_canister::list_neurons::Args {
        limit: 100,
        start_page_at: None,
        of_principal: None,
    };

    ic_cdk::println!("Fetching neuron data");
    match sns_governance_canister_c2c_client::list_neurons(canister_id, &args).await {
        Ok(response) => {
            PRINCIPAL_NEURONS.with(|prin| {
                MATURITY_HISTORY.with(|hist| {
                    NEURON_REGISTRY.with(|reg| {
                        let mut reg_borrow = reg.borrow_mut();
                        let mut hist_borrow = hist.borrow_mut();
                        let mut prin_borrow = prin.borrow_mut();

                        response.neurons.iter().for_each(|neuron| {
                            update_registry(&mut reg_borrow, neuron);
                            update_history(&mut hist_borrow, neuron);
                            update_principal_mapping(&mut prin_borrow, neuron);
                        });
                    });
                })
            });
            Ok(response.neurons.len() as u64)
        }
        Err(err) => {
            ic_cdk::println!("err: {:?}", err);
            Err(format!("Error: {:?}", err))
        }
    }
}
// Function to update registry
fn update_registry(reg: &mut BTreeMap<NeuronId, NeuronInfo>, neuron: &Neuron) {
    if let Some(id) = &neuron.id {
        let maturity = neuron.maturity_e8s_equivalent;
        match reg.entry(id.clone()) {
            btree_map::Entry::Vacant(entry) => {
                entry.insert(NeuronInfo {
                    last_maturity: maturity,
                    accumulated_maturity: 0,
                });
            }
            btree_map::Entry::Occupied(mut entry) => {
                let info = entry.get_mut();
                info.accumulated_maturity += maturity - info.last_maturity;
                info.last_maturity = maturity;
            }
        }
    }
}

// Function to update history
fn update_history(hist: &mut BTreeMap<NeuronId, Vec<(u64, u64)>>, neuron: &Neuron) {
    if let Some(id) = &neuron.id {
        let maturity = neuron.maturity_e8s_equivalent;
        match hist.entry(id.clone()) {
            btree_map::Entry::Vacant(entry) => {
                entry.insert(vec![(timestamp_seconds(), maturity)]);
            }
            btree_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push((timestamp_seconds(), maturity));
            }
        }
    }
}

// Function to update principal-neuron mapping
fn update_principal_mapping(prin: &mut BTreeMap<Principal, Vec<NeuronId>>, neuron: &Neuron) {
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

pub type GetNeuronResponse = NeuronInfo;

#[query]
fn get_neuron(id: NeuronId) -> Option<GetNeuronResponse> {
    NEURON_REGISTRY.with(|reg| { reg.borrow().get(&id).cloned() })
}

#[query]
fn get_principal_neurons(principal: Principal) -> Option<Vec<NeuronId>> {
    PRINCIPAL_NEURONS.with(|prin| { prin.borrow().get(&principal).cloned() })
}

#[query]
fn get_all_principals_and_number_of_neurons() -> Vec<(Principal, u64)> {
    PRINCIPAL_NEURONS.with(|prin| {
        prin.borrow()
            .iter()
            .map(|(p, n)| (p.clone(), n.len() as u64))
            .collect()
    })
}
