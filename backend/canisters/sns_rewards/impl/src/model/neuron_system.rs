use crate::model::maturity_history::MaturityHistory;
use crate::state::SyncInfo;
use serde::Deserialize;
use serde::Serialize;
use sns_governance_canister::types::{Neuron, NeuronId};
use std::collections::BTreeMap;
use std::collections::{btree_map, HashMap};
use tracing::debug;
use types::NeuronInfo;

#[derive(Serialize, Deserialize, Default)]
pub struct NeuronSystem {
    /// Information about periodic synchronization
    pub sync_info: SyncInfo,
    /// Internal accounting: accumulated maturity and reward history
    pub neuron_maturity: BTreeMap<NeuronId, NeuronInfo>,
    /// The history of each neuron's maturity.
    pub maturity_history: MaturityHistory,
}

// TODO: add here check that neuron is eligible for GLDT rewards
impl NeuronSystem {
    // Save the neuron stake info and update total staked power accordingly
    pub fn upsert_neuron(&mut self, neuron: &Neuron) {
        // Save the neuron maturity info and total maturity accordingly
        if let Some(id) = &neuron.id {
            if !neuron.is_reward_eligible() {
                // If the neuron is no longer eligible, we remove it from the active
                // tracking map so it doesn't accrue rewards in the next distribution.
                if self.neuron_maturity.remove(id).is_some() {
                    debug!(
                        "Neuron {} removed from system: no longer eligible for rewards.",
                        id
                    );
                }
                return;
            }

            let updated_neuron: Option<(NeuronId, NeuronInfo)>;

            let maturity = neuron.calculate_total_maturity();

            let neuron_info = NeuronInfo {
                last_synced_maturity: maturity,
                accumulated_maturity: 0,
                rewarded_maturity: HashMap::new(),
                last_disburse_event_considered: Some(0),
            };

            // TODO - check age of neuron to avoid someone gaming the system by spawning neurons (check if really relevant)
            match self.neuron_maturity.entry(id.clone()) {
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(neuron_info.clone());
                    updated_neuron = Some((id.clone(), neuron_info));
                }
                btree_map::Entry::Occupied(mut entry) => {
                    let neuron_info_entry = entry.get_mut();
                    let spawning_maturity =
                        extract_and_sum_up_new_disburse_events(neuron, neuron_info_entry);
                    println!("spawning_maturity {spawning_maturity}");
                    let total_maturity = maturity + spawning_maturity;
                    println!("total_maturity {total_maturity}");

                    if let Some(delta) =
                        total_maturity.checked_sub(neuron_info_entry.last_synced_maturity)
                    {
                        // only add the difference if the maturity has increased
                        if delta == 0 {
                            return;
                        }
                        // update accumulated maturity
                        neuron_info_entry.accumulated_maturity = neuron_info_entry
                            .accumulated_maturity
                            .checked_add(delta)
                            .unwrap_or(neuron_info_entry.accumulated_maturity);
                    }
                    // update the last_synced_maturity
                    neuron_info_entry.last_synced_maturity = maturity;
                    updated_neuron = Some((id.clone(), neuron_info_entry.clone()));
                }
            }
            // update history
            if let Some((n_id, n_info)) = updated_neuron {
                self.maturity_history
                    .insert((n_id, self.sync_info.last_synced_start), n_info)
            }
        }
    }
}

// Function to update principal-neuron mapping
fn extract_and_sum_up_new_disburse_events(neuron: &Neuron, neuron_info: &mut NeuronInfo) -> u64 {
    let mut last_disburse_event_considered =
        neuron_info.last_disburse_event_considered.unwrap_or(0);
    let total_from_new_disburse_events =
        neuron
            .disburse_maturity_in_progress
            .iter()
            .fold(0u64, |mut acc, event| {
                if event.timestamp_of_disbursement_seconds
                    > neuron_info.last_disburse_event_considered.unwrap_or(0)
                {
                    acc += event.amount_e8s;
                }
                if event.timestamp_of_disbursement_seconds > last_disburse_event_considered {
                    last_disburse_event_considered = event.timestamp_of_disbursement_seconds;
                }
                acc
            });

    neuron_info.last_disburse_event_considered = Some(last_disburse_event_considered);
    println!("total_from_new_disburse_events {total_from_new_disburse_events}");
    total_from_new_disburse_events
}
