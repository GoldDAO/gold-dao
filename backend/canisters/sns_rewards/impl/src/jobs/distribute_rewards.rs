/*!
# SNS reward distribution

This job is responsible for distributing rewards to user's sub accounts.
All the different reward tokens are to be held in the 0 sub account.
*/

use canister_time::{now_millis, run_now_then_interval, WEEK_IN_MS};
use sns_governance_canister::types::NeuronId;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use types::Milliseconds;
use crate::{
    model::maturity_history::MaturityHistory,
    state::{mutate_state, read_state},
};

const DISTRIBUTION_INTERVAL: Milliseconds = WEEK_IN_MS;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(DISTRIBUTION_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(distribute_rewards())
}

pub async fn distribute_rewards() {
    // STEPS
    // 1 [TICK] Calculate Total maturity 
    // 2 [TICK] get the percentage payout for each neuron
    // 3 ) Get total OGY, ICP, GLDGov in sub account 0
    // 4 ) Pay each sub account its perentage of OGY, ICP, GLDGov

    // part 1 ) Cacluating total maturity
    let new_distribution_time = now_millis();
    mutate_state(|state| {
        state.data.sync_info.last_distribution_start = new_distribution_time;
    });

    let last_distribution_end_time = read_state(|state| state.data.sync_info.last_distribution_end).clone();
    

    let neuron_maturity_for_interval = read_state(|state| {
        calculate_neuron_maturity_for_interval(&state.data.maturity_history)
    });

    let total_maturity_for_all_neurons : u64 = neuron_maturity_for_interval.iter().map(|entry| entry.1).sum();

    let neuron_reward_percentage : Vec<(NeuronId, u64)> = neuron_maturity_for_interval.iter().map(|entry| {
        let percentage = entry.1.checked_div(total_maturity_for_all_neurons).unwrap_or(0);
        (entry.0.clone(), percentage)
    }).collect();


}



pub fn calculate_neuron_maturity_for_interval(
    maturity_history: &MaturityHistory,
) -> Vec<(NeuronId, u64)> {
    let mut latest_maturity_per_neuron: Vec<(NeuronId, u64)> = Vec::new();

 // 1 day to act as the previous maturity and the rest to act as 7 days of maturity.
    // loop over all neurons
    for neuron_id in maturity_history.get_keys() {
        // get the latest entry 
        let history = maturity_history.get_maturity_history(neuron_id.clone(), 1); 

        // previous weekly maturity is always the first entry
        let latest_entry = history.first().expect("There is no history for this neuron");

        let accumilated_maturity = latest_entry.1.accumulated_maturity; // total accumilated
        let previous_paid_maturity = latest_entry.1.rewarded_maturity; // last payout reward
        
        let change_since_last_interval = accumilated_maturity.checked_sub(previous_paid_maturity).expect("overflow when subtracting"); 
        latest_maturity_per_neuron.push((neuron_id.clone(), change_since_last_interval));
    }

    latest_maturity_per_neuron
}

#[cfg(test)]
mod tests {}
