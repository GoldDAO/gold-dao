use candid::Principal;
use canister_time::{now_millis, run_now_then_interval, DAY_IN_MS};
use sns_governance_canister::types::{Neuron, NeuronId};
use std::collections::BTreeMap as NormalBTreeMap;
use std::time::Duration;
use tracing::{debug, error, info};
use types::Milliseconds;

use crate::{
    jobs::sync_supply_data::{self},
    state::{mutate_state, read_state, GovernanceStats},
};

const SYNC_NEURONS_INTERVAL: Milliseconds = DAY_IN_MS;

pub fn start_job() {
    debug!("Starting the governance sync job..");
    run_now_then_interval(Duration::from_millis(SYNC_NEURONS_INTERVAL), run)
}

pub fn run() {
    ic_cdk::spawn(sync_neurons_data())
}

pub async fn sync_neurons_data() {
    info!("sync_neurons_data..");
    let canister_id = read_state(|state| state.data.sns_governance_canister);

    mutate_state(|state| {
        state.data.sync_info.last_synced_start = now_millis();
    });

    let mut number_of_scanned_neurons = 0;
    let mut continue_scanning: bool = true;

    let mut args = sns_governance_canister::list_neurons::Args {
        limit: 100,
        start_page_at: None,
        of_principal: None,
    };

    // We want new empty structures when re-computing the data, otherwise it will
    // sum up with data from previous job
    // Q: BTreeMap is a stable structure? Do we want that for a temp variable like this?
    let mut temp_principal_with_neurons: NormalBTreeMap<Principal, Vec<NeuronId>> =
        NormalBTreeMap::new();
    let mut temp_principal_with_stats: NormalBTreeMap<Principal, GovernanceStats> =
        NormalBTreeMap::new();
    let mut temp_all_gov_stats: GovernanceStats = GovernanceStats::default();

    while continue_scanning {
        continue_scanning = false;

        match sns_governance_canister_c2c_client::list_neurons(canister_id, &args).await {
            Ok(response) => {
                // info!("{:?}", response);
                // Mutate the state to update the principal with governance data
                info!("Iterating neurons");
                response.neurons.iter().for_each(|neuron| {
                    update_principal_neuron_mapping(
                        &mut temp_principal_with_neurons,
                        &mut temp_principal_with_stats,
                        &mut temp_all_gov_stats,
                        neuron,
                    );
                    // update_neuron_maturity(state, neuron);
                });

                // Check if we hit the end of the list
                let number_of_received_neurons = response.neurons.len();
                if ((number_of_received_neurons as u32) == 100) {
                    args.start_page_at = response.neurons.last().map_or_else(
                        || {
                            error!("we should not be here, last neurons from response is missing?");
                            None
                        },
                        |n| {
                            continue_scanning = true;
                            n.id.clone()
                        },
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
        state.data.sync_info.last_synced_end = now_millis();
        state.data.sync_info.last_synced_number_of_neurons = number_of_scanned_neurons;

        let principal_with_neurons = &mut state.data.principal_neurons;
        let principal_with_stats = &mut state.data.principal_gov_stats;
        let all_gov_stats = &mut state.data.all_gov_stats;

        // Update the state with the newly computed data
        principal_with_neurons.clear();
        principal_with_stats.clear();
        for (key, value) in temp_principal_with_neurons {
            principal_with_neurons.insert(key, value);
        }
        for (key, value) in temp_principal_with_stats {
            principal_with_stats.insert(key, value);
        }
        *all_gov_stats = temp_all_gov_stats;
    });

    // After we have computed governance stats, update the total supply and circulating supply
    sync_supply_data::run();
}
fn update_principal_neuron_mapping(
    principal_with_neurons: &mut NormalBTreeMap<Principal, Vec<NeuronId>>,
    principal_with_stats: &mut NormalBTreeMap<Principal, GovernanceStats>,
    all_gov_stats: &mut GovernanceStats,
    neuron: &Neuron,
) {
    if let Some(permissioned_principal) = neuron.permissions.first() {
        if let Some(pid) = permissioned_principal.principal {
            // Update the array of neurons in the principal_neurons map
            principal_with_neurons
                .entry(pid)
                .and_modify(|neurons| {
                    if let Some(id) = &neuron.id {
                        if !neurons.contains(id) {
                            neurons.push(id.clone());
                        }
                    }
                })
                .or_insert_with(|| {
                    if let Some(id) = &neuron.id {
                        vec![id.clone()]
                    } else {
                        vec![]
                    }
                });

            let neuron_staked_maturity = neuron.staked_maturity_e8s_equivalent.unwrap_or(0);
            // Update the governance stats of the Principal
            principal_with_stats
                .entry(pid)
                .and_modify(|stats| {
                    // Total staked is how much the principal staked at the begginging + how much of maturity they restaked
                    stats.total_staked += neuron.cached_neuron_stake_e8s
                        + neuron_staked_maturity
                        + neuron.maturity_e8s_equivalent;
                    // Total locked is the amount of tokens they have staked
                    stats.total_locked += neuron.cached_neuron_stake_e8s + neuron_staked_maturity;
                    // Total unlocked is `maturity_e8s_equivalent` which can be claimed
                    stats.total_unlocked += neuron.maturity_e8s_equivalent;
                    // Total rewards is what they have as maturity and what they have as staked_maturity
                    stats.total_rewards += neuron_staked_maturity + neuron.maturity_e8s_equivalent
                })
                .or_insert_with(|| GovernanceStats {
                    total_staked: neuron.cached_neuron_stake_e8s
                        + neuron_staked_maturity
                        + neuron.maturity_e8s_equivalent,
                    total_locked: neuron.cached_neuron_stake_e8s + neuron_staked_maturity,
                    total_unlocked: neuron.maturity_e8s_equivalent,
                    total_rewards: neuron_staked_maturity + neuron.maturity_e8s_equivalent,
                });
            all_gov_stats.total_staked += neuron.cached_neuron_stake_e8s
                + neuron_staked_maturity
                + neuron.maturity_e8s_equivalent;
            all_gov_stats.total_locked += neuron.cached_neuron_stake_e8s + neuron_staked_maturity;
            all_gov_stats.total_unlocked += neuron.maturity_e8s_equivalent;
            all_gov_stats.total_rewards += neuron_staked_maturity + neuron.maturity_e8s_equivalent;
        }
    }
}
