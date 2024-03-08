/*!
# SNS reward distribution

This job is responsible for distributing rewards to user's sub accounts.
All the different reward tokens are to be held in the 0 sub account.
*/

use candid::{CandidType, Nat, Principal};
use canister_time::{now_millis, run_interval, WEEK_IN_MS};
use ic_ledger_types::Subaccount;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use types::{Milliseconds, Token};
use ic_cdk::api::call::{call, RejectionCode};
use ic_cdk::{ api};
use crate::{
    model::maturity_history::MaturityHistory,
    state::{mutate_state, read_state},
};

const DISTRIBUTION_INTERVAL: Milliseconds = WEEK_IN_MS;

pub fn start_job() {
    run_interval(Duration::from_millis(DISTRIBUTION_INTERVAL), run);
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

    // 1 ) Cacluating neuron reward percentage
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


    // 2 ) Get balances of all reward pools
    let ogy_reward_pool_id = read_state(|state| state.data.ogy_ledger_canister);
    let ogy_reward_pool_balance = query_token_balance_icrc1(ogy_reward_pool_id).await;



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

#[derive(CandidType, Serialize, Deserialize)]
struct BalanceQuery {
    owner: Principal,
    subaccount : Option<Subaccount>
}

#[derive(CandidType, Deserialize)]
struct BalanceResponse(u64);



async fn query_token_balance_icrc1(ledger_id : Principal) -> Result<u64, String> {
    let a = BalanceQuery {
        owner: ic_cdk::api::id(),
        subaccount: None, // Adjust according to your data type
    };

    let (b_res,) = call(ledger_id, "icrc1_balance_of", (a,)).await.unwrap();
    b_res
}

#[cfg(test)]
mod tests {}
