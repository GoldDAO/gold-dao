/*!
# SNS reward distribution

This job is responsible for distributing rewards to user's sub accounts.
All the different reward tokens are to be held in the 0 sub account.
*/

use crate::{
    model::maturity_history::MaturityHistory,
    state::{mutate_state, read_state},
};
use candid::{CandidType, Nat, Principal};
use canister_time::{now_millis, run_interval, WEEK_IN_MS};
use ic_cdk::api;
use ic_cdk::api::call::{call, RejectionCode};
use ic_ledger_types::Subaccount;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use std::collections::BTreeMap;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use types::{Milliseconds, NeuronInfo, Token};

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

    let new_distribution_time = now_millis();
    mutate_state(|state| {
        state.data.sync_info.last_distribution_start = new_distribution_time;
    });

    // let last_distribution_end_time = read_state(|state| state.data.sync_info.last_distribution_end).clone();

    // 1 ) Cacluating neuron reward percentage
    let neuron_maturity_for_interval = read_state(|state| {
        calculate_neuron_maturity_for_interval(
            &state.data.neuron_maturity,
            &state.data.maturity_history,
        )
    });
    let total_maturity_for_all_neurons =
        calculate_aggregated_maturity(&neuron_maturity_for_interval);
    let neuron_reward_percentage = calculate_neuron_reward_percentages(
        &neuron_maturity_for_interval,
        &total_maturity_for_all_neurons,
    );

    // 2 ) Get balances of all reward pools
    let ogy_reward_pool_id = read_state(|state| state.data.ogy_ledger_canister);
    let ogy_reward_pool_balance = query_token_balance_icrc1(ogy_reward_pool_id).await;

    // 3 ) Pay each sub account & update neuron history
}

pub fn calculate_neuron_maturity_for_interval(
    neuron_maturity: &BTreeMap<NeuronId, NeuronInfo>,
    maturity_history: &MaturityHistory,
) -> Vec<(NeuronId, u64)> {
    let mut latest_maturity_per_neuron: Vec<(NeuronId, u64)> = Vec::new();

    // 1 day to act as the previous maturity and the rest to act as 7 days of maturity.
    // loop over all neurons
    for (neuron_id, neuron_info) in neuron_maturity.iter() {

        let accumilated_maturity = neuron_info.accumulated_maturity; // total accumilated
        let previous_paid_maturity = neuron_info.rewarded_maturity; // last payout reward

        let change_since_last_interval = accumilated_maturity
            .checked_sub(previous_paid_maturity)
            .expect("overflow when subtracting");
        latest_maturity_per_neuron.push((neuron_id.clone(), change_since_last_interval));
    }

    latest_maturity_per_neuron
}

pub fn calculate_aggregated_maturity(data: &Vec<(NeuronId, u64)>) -> u64 {
    data.iter().map(|entry| entry.1).sum()
}

pub fn calculate_neuron_reward_percentages(
    data: &Vec<(NeuronId, u64)>,
    total_maturity: &u64,
) -> Vec<(NeuronId, u64)> {
    data.iter()
        .map(|entry| {
            let percentage = entry.1.checked_div(total_maturity.clone()).unwrap_or(0);
            (entry.0.clone(), percentage)
        })
        .collect()
}

#[derive(CandidType, Serialize, Deserialize)]
struct BalanceQuery {
    owner: Principal,
    subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize)]
struct BalanceResponse(u64);

async fn query_token_balance_icrc1(ledger_id: Principal) -> Result<u64, String> {
    let a = BalanceQuery {
        owner: ic_cdk::api::id(),
        subaccount: None, // Adjust according to your data type
    };

    let (b_res,) = call(ledger_id, "icrc1_balance_of", (a,)).await.unwrap();
    b_res
}

#[cfg(test)]
mod tests {
    use candid::Principal;
    use sns_governance_canister::types::{Neuron, NeuronId, NeuronPermission};
    use types::NeuronInfo;

    use crate::{
        jobs::synchronise_neurons::update_neuron_maturity,
        state::{init_state, mutate_state, read_state, RuntimeState},
    };

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    use super::calculate_neuron_maturity_for_interval;

    #[test]
    fn test_calculate_neuron_maturity_for_interval() {
        init_runtime_state();

        // ********************************
        // 1. Insert new neuron and update its matuirty 8 times to act as a week's worth of data
        // ********************************

        //////////////////////////////////////////////
        // neuron 1
        //////////////////////////////////////////////
        let neuron_id_1 =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();
        let mut neuron_1 = Neuron::default();
        neuron_1.id = Some(neuron_id_1.clone());

        mutate_state(|state| {
            update_neuron_maturity(state, &neuron_1);
        });

        // day 1
        neuron_1.maturity_e8s_equivalent = 100;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });
        // dat 2
        neuron_1.maturity_e8s_equivalent = 200;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });
        // day 3
        neuron_1.maturity_e8s_equivalent = 300;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });
        // day 4
        neuron_1.maturity_e8s_equivalent = 400;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });
        // day 5
        neuron_1.maturity_e8s_equivalent = 500;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });
        // day 6
        neuron_1.maturity_e8s_equivalent = 600;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });
        // day 7
        neuron_1.maturity_e8s_equivalent = 700;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });
        // day 5
        neuron_1.maturity_e8s_equivalent = 800;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });

        // calculate_neuron_maturity_for_interval
        read_state(|state| {
            let d = calculate_neuron_maturity_for_interval(
                &state.data.neuron_maturity,
                &state.data.maturity_history,
            );
            let maturity_for_interval = d.get(0).unwrap().1;
            assert_eq!(maturity_for_interval, 850);
        })
    }

    #[test]
    fn test_calculate_neuron_maturity_for_interval_second_cycle() {
        init_runtime_state();

        // ********************************
        // 1. Insert new neuron and update its matuirty 8 times to act as a week's worth of data
        // ********************************

        //////////////////////////////////////////////
        // neuron 1 - week 1
        //////////////////////////////////////////////
        let neuron_id_1 =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();
        let mut neuron_1 = Neuron::default();
        neuron_1.id = Some(neuron_id_1.clone());

        // day 1
        mutate_state(|state| {
            update_neuron_maturity(state, &neuron_1);
        });

        // day 2
        neuron_1.maturity_e8s_equivalent = 100;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });
        // day 3
        neuron_1.maturity_e8s_equivalent = 150;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });

        // calculate_neuron_maturity_for_interval for week 1
        read_state(|state| {
            let d = calculate_neuron_maturity_for_interval(
                &state.data.neuron_maturity,
                &state.data.maturity_history,
            );
            let maturity_for_interval = d.get(0).unwrap().1;
            assert_eq!(maturity_for_interval, 200);
        });

        // fake paying the first week.
        mutate_state(|state| {
            // state
            //     .data
            //     .maturity_history
            //     .set_rewarded_on_latest_entry(&neuron_id_1, 200);
            
            state.data.neuron_maturity.get_mut(&neuron_id_1).unwrap().rewarded_maturity = 200;

        });

        // verify the latest entry for a neuron has the payment
        read_state(|state| {
            let neuron = state
                .data
                .neuron_maturity
                .get(&neuron_id_1)
                .unwrap();
            assert_eq!(neuron.rewarded_maturity, 200);
        });


        //////////////////////////////////////////////
        // neuron 1 - week 2
        //////////////////////////////////////////////

        // add more maturity
        neuron_1.maturity_e8s_equivalent = 200;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            state.data.sync_info.last_synced_start += 100;
            update_neuron_maturity(state, &neuron_1);
        });


        // calcualte maturity for the second week
        read_state(|state| {
            let d = calculate_neuron_maturity_for_interval(
                &state.data.neuron_maturity,
                &state.data.maturity_history,
            );
            let maturity_for_interval = d.get(0).unwrap().1;
            assert_eq!(maturity_for_interval, 50);
        });
    }
}
