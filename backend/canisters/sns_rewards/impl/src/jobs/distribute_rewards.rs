/*!
# SNS reward distribution

This job is responsible for distributing rewards to user's sub accounts.
All the different reward tokens are to be held in the 0 sub account.
*/

use crate::state::{mutate_state, read_state, RuntimeState};
use candid::{CandidType, Principal};
use canister_time::{now_millis, run_interval, WEEK_IN_MS};
use futures::future::join_all;
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, Subaccount, Tokens, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use std::time::Duration;
use std::collections::BTreeMap;
use tracing::{debug, info};
use types::{Milliseconds, NeuronInfo};

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
    let time_start = now_millis();
    mutate_state(|state| {
        state.data.sync_info.last_distribution_start = time_start;
    });

    // 1 ) Cacluating neuron reward percentage
    let neuron_maturity_for_interval =
        read_state(|state| calculate_neuron_maturity_for_interval(&state.data.neuron_maturity));

    let total_maturity_for_all_neurons =
        calculate_aggregated_maturity(&neuron_maturity_for_interval);

    let neuron_reward_percentage = calculate_neuron_percentages(
        &neuron_maturity_for_interval,
        &total_maturity_for_all_neurons,
    );

    // 2 ) Get balances of all reward pools
    // let ogy_reward_pool_id = read_state(|state| state.data.ogy_ledger_canister);
    // let ogy_reward_pool_balance = query_token_balance_icrc1(ogy_reward_pool_id).await;
    let fake_ledger_id = Principal::from_slice(&[0, 0, 0, 0, 1, 112, 26, 234, 1, 1]);
    info!("icp ledger id {}", fake_ledger_id);
    let icp_reward_pool_balance = get_icp_balance().await;

    info!(
        "Balance of ICP in reward pool is : {}",
        icp_reward_pool_balance
    );

    let reduced_icp_rewards: u64 = 300_000;

    info!("we will transfer : {}", reduced_icp_rewards);

    // 4 ) Pay all sub accounts
    let sucessful_neuron_transfers =
        transfer_rewards(neuron_reward_percentage, reduced_icp_rewards).await;

    debug!("rewards transfered");
    // update the neuron info with the amount of maturity paid
    mutate_state(|state| {
        update_neuron_reward(
            &sucessful_neuron_transfers,
            state,
            &neuron_maturity_for_interval,
        );
    });

    let time_finish = now_millis();
    mutate_state(|state| state.data.sync_info.last_distribution_end = time_finish);
    info!(
        "|||| DISTRIBUTION COMPLETE ||||| time_taken : {} || number of neurons distributed to : {}",
        (time_finish - time_start),
        &sucessful_neuron_transfers.len()
    );
}

pub fn calculate_reward(percentage : BigUint, reward_pool : u64) -> u64 {
    let reward = (BigUint::from(reward_pool) * percentage.clone()) / BigUint::from(100_000_000u64);
    reward.try_into().expect("faild to convert bigint to u64")
}

pub fn update_neuron_reward(
    neuron_ids_to_update: &Vec<NeuronId>,
    state: &mut RuntimeState,
    neuron_interval_maturity: &Vec<(NeuronId, u64)>,
) {
    for neuron_id in neuron_ids_to_update {
        let neuron = state.data.neuron_maturity.get_mut(&neuron_id);
        match neuron {
            Some(neuron_info) => {
                debug!(
                    "current reward maturity : {}",
                    neuron_info.rewarded_maturity
                );

                let neuron_maturity = neuron_interval_maturity
                    .iter()
                    .find(|(n_id, _)| n_id.clone() == neuron_id.clone());
                match neuron_maturity {
                    Some((_, n_mat)) => {
                        let new_rewarded_maturity = neuron_info.rewarded_maturity + n_mat.clone();
                        debug!(
                            "updating neuron maturity : {} with maturity reward of {}",
                            neuron_id, new_rewarded_maturity
                        );
                        neuron_info.rewarded_maturity = new_rewarded_maturity;
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}

pub fn calculate_neuron_maturity_for_interval(
    neuron_maturity: &BTreeMap<NeuronId, NeuronInfo>,
) -> Vec<(NeuronId, u64)> {
    let mut latest_maturity_per_neuron: Vec<(NeuronId, u64)> = Vec::new();

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

// pub fn calculate_neuron_reward_percentages(
//     data: &Vec<(NeuronId, u64)>,
//     total_maturity: &u64,
// ) -> Vec<(NeuronId, f64)> {
//     data.iter()
//         .map(|entry| {
//             let percentage = entry.1 as f64 / total_maturity.clone() as f64;
//             (entry.0.clone(), percentage)
//         })
//         .collect()
// }

pub fn calculate_neuron_percentages(
    data: &[(NeuronId, u64)],
    total_maturity: &u64,
) -> Vec<(NeuronId, BigUint)> {
    // Convert total_maturity to BigUint
    let total_maturity_big = BigUint::try_from(total_maturity.clone()).unwrap();

    // Calculate percentage for each neuron
    data.iter()
        .map(|(neuron_id, maturity)| {
            // Convert maturity to BigUint
            let maturity_big = BigUint::try_from(*maturity).unwrap();

            // Calculate percentage as (maturity / total_maturity) * 10000 (expressed in basis points)
            let percentage =
                (maturity_big * BigUint::from(100_000_000u64)) / total_maturity_big.clone();

            (neuron_id.clone(), percentage)
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

async fn get_icp_balance() -> Tokens {
    ic_ledger_types::account_balance(
        MAINNET_LEDGER_CANISTER_ID,
        AccountBalanceArgs {
            account: AccountIdentifier::new(&ic_cdk::api::id(), &DEFAULT_SUBACCOUNT),
        },
    )
    .await
    .expect("call to ledger failed")
}

async fn transfer_icp_to_sub_account(sub_account: Subaccount, amount: u64) -> Result<u64, String> {
    let fake_ledger_id = Principal::from_slice(&[0, 0, 0, 0, 1, 112, 26, 234, 1, 1]);
    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        fake_ledger_id,
        &(TransferArg {
            from_subaccount: Some(DEFAULT_SUBACCOUNT.0),
            to: Account {
                owner: ic_cdk::api::id(),
                subaccount: Some(sub_account.0),
            },
            fee: Some((10_000u32).into()),
            created_at_time: None,
            amount: (amount).into(),
            memo: None,
        }),
    )
    .await
    {
        Ok(Ok(_)) => {
            debug!("!!! TRANSFER SUCCESS !!!");
            Ok(1)
        }
        Ok(Err(error)) => {
            debug!("!!! TRANSFER ERROR!!! error : {}", error);
            return Err(format!("Transfer error: {error:?}"));
        }
        Err(error) => {
            debug!("!!! TRANSFER ERROR!!! error : {}", error.1);
            return Err(format!("Network error: {error:?}"));
        }
    }
}

async fn transfer_rewards(neurons: Vec<(NeuronId, BigUint)>, icp_balance: u64) -> Vec<NeuronId> {
    let mut successful_reward_transfers: Vec<NeuronId> = vec![];
    let mut number_of_valid_neurons = 0;

    // Split neurons into batches of 50
    let mut batched_neurons = neurons.chunks(50);

    // Process each batch sequentially
    while let Some(batch) = batched_neurons.next() {
        let mut transfer_futures = Vec::new();

        for (neuron_id, percentage_to_reward) in batch {
            if *percentage_to_reward <= BigUint::from(0u64) {
                continue; // skip payment since this neuron 0 percentage
            }
            number_of_valid_neurons += 1;
            debug!(
                "transfering for neuron id : {} || percentage to reward : {} || ",
                neuron_id, percentage_to_reward
            );
            // sub account id
            let neuron_id_as_bytes = neuron_id
                .clone()
                .into_array()
                .expect("Error converting NeuronId into u8");
            let sub_account = Subaccount(neuron_id_as_bytes);

            // handle ICP transfer
            // let icp_reward = ((icp_balance as f64) * percentage_to_reward).floor() as u64;
            let icp_reward = calculate_reward(percentage_to_reward.clone(), icp_balance);
            debug!("icp reward : {}", icp_reward);

            let icp_transfer_future = transfer_icp_to_sub_account(sub_account, icp_reward);
            transfer_futures.push((neuron_id.clone(), icp_transfer_future));
        }

        // Execute all transfer futures concurrently and collect results
        let results = join_all(transfer_futures.into_iter().map(
            |(neuron_id, future)| async move {
                match future.await {
                    Ok(_) => Ok(neuron_id),
                    Err(e) => {
                        debug!(
                            "!!error in transfer - neuron_id ::: {} - error ::: {}",
                            neuron_id, e
                        );
                        Err(())
                    } // Handle error if needed
                }
            },
        ))
        .await;

        // Collect successful transfers
        for result in results {
            match result {
                Ok(neuron_id) => {
                    successful_reward_transfers.push(neuron_id);
                }
                Err(_) => {
                    // Handle error if needed
                }
            }
        }
    }

    debug!(
        "||| number of neurons that should receive a payment : {} |||",
        number_of_valid_neurons
    );
    successful_reward_transfers
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use sns_governance_canister::types::{Neuron, NeuronId};

    use crate::{
        jobs::{
            distribute_rewards::{
                calculate_aggregated_maturity, calculate_neuron_percentages, calculate_reward, update_neuron_reward
            },
            synchronise_neurons::update_neuron_maturity,
        },
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
            let d = calculate_neuron_maturity_for_interval(&state.data.neuron_maturity);
            let maturity_for_interval = d.get(0).unwrap().1;
            assert_eq!(maturity_for_interval, 850);
        })
    }

    #[test]
    fn test_neuron_percentages() {
        // Example data with 200 neurons
        // Generate 200 neurons with unique IDs and a single maturity value
        let mut neuron_data: Vec<(NeuronId, u64)> = Vec::new();
        for _ in 0..20000 {
            let neuron_id =
                NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                    .unwrap();
            neuron_data.push((neuron_id, 1)); // Using 100 as the maturity value for each neuron
        }

        // Example total_maturity
        let total_maturity: u64 = neuron_data.iter().map(|(_, m)| *m).sum();

        // Calculate neuron percentages
        let neuron_percentages = calculate_neuron_percentages(&neuron_data, &total_maturity);

        // Ensure the sum of percentages equals 10000 (total basis points)
        let sum_percentages: BigUint = neuron_percentages.iter().map(|(_, b)| b.clone()).sum();

        let expected_sum = BigUint::from(100_000_000u64);
        // let tolerance = BigUint::from(1u64); // Adjust tolerance as needed

        assert_eq!(expected_sum, sum_percentages);
        // let only_percentages: Vec<BigUint> =
        //     neuron_percentages.iter().map(|(a, b)| b.clone()).collect();

        // Print the result for verification
    }

    #[test]
    fn test_rewards_calculation() {
        // Example data with 200 neurons, each with a unique ID and maturity of 100
        let mut neuron_data: Vec<(NeuronId, u64)> = Vec::new();
        for _ in 0..200 {
            let neuron_id =
                NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                    .unwrap();
            neuron_data.push((neuron_id, 100));
        }

        // Example total_maturity
        let total_maturity: u64 = neuron_data.iter().map(|(_, m)| *m).sum();

        // Calculate neuron percentages
        let neuron_percentages = calculate_neuron_percentages(&neuron_data, &total_maturity);

        // Example reward pool
        let reward_pool = 10_000_000u64;

        // Calculate rewards based on percentages
        let rewards: Vec<(NeuronId, u64)> = neuron_percentages
            .iter()
            .map(|(neuron_id, percentage)| {
                let reward = calculate_reward(percentage.clone(), reward_pool);
                (neuron_id.clone(), reward)
            })
            .collect();

        // Check if the sum of rewards equals the reward pool
        let sum_rewards: u64 = rewards.iter().map(|(_, reward)| *reward).sum();

        // let only_rewards: Vec<u64> = rewards.iter().map(|(_, b)| b.clone()).collect();

        assert_eq!(
            sum_rewards, reward_pool,
            "Sum of rewards should equal the reward pool"
        );
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
            let d = calculate_neuron_maturity_for_interval(&state.data.neuron_maturity);
            let maturity_for_interval = d.get(0).unwrap().1;
            assert_eq!(maturity_for_interval, 200);
        });

        // fake paying the first week.
        mutate_state(|state| {
            state
                .data
                .neuron_maturity
                .get_mut(&neuron_id_1)
                .unwrap()
                .rewarded_maturity = 200;
        });

        // verify the latest entry for a neuron has the payment
        read_state(|state| {
            let neuron = state.data.neuron_maturity.get(&neuron_id_1).unwrap();
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
            let d = calculate_neuron_maturity_for_interval(&state.data.neuron_maturity);
            let maturity_for_interval = d.get(0).unwrap().1;
            assert_eq!(maturity_for_interval, 50);
        });
    }

    #[test]
    fn test_calculate_aggregated_maturity() {
        let neuron_id_1 =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();
        let neuron_id_2 =
            NeuronId::new("3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f97")
                .unwrap();
        let neuron_id_3 =
            NeuronId::new("4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f96")
                .unwrap();

        let neuron_list = vec![(neuron_id_1, 100), (neuron_id_2, 200), (neuron_id_3, 300)];

        assert_eq!(calculate_aggregated_maturity(&neuron_list), 600);
    }

    #[test]
    fn test_calculate_neuron_reward_percentages() {
        let neuron_id_1 =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();
        let neuron_id_2 =
            NeuronId::new("3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f97")
                .unwrap();
        let neuron_id_3 =
            NeuronId::new("4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f96")
                .unwrap();

        let neuron_list = vec![(neuron_id_1, 100), (neuron_id_2, 200), (neuron_id_3, 300)];

        let total_maturity = calculate_aggregated_maturity(&neuron_list);

        let maturity_percentage_per_neuron =
        calculate_neuron_percentages(&neuron_list, &total_maturity);
        let expected_vals: Vec<BigUint> = vec![BigUint::from(16666666u64), BigUint::from(33333333u64), BigUint::from(50000000u64)];

        let test_run: Vec<((NeuronId, BigUint), BigUint)> = maturity_percentage_per_neuron
            .iter()
            .zip(expected_vals.iter())
            .map(|((neuron_id, percentage), expected_val)| {
                ((neuron_id.clone(), percentage.clone()), expected_val.clone())
            })
            .collect();

        for ((_, percentage), expected_val) in test_run {
            assert_eq!(percentage, expected_val);
        }
    }

    #[test]
    fn test_update_neuron_maturity() {
        init_runtime_state();

        let neuron_id_1 =
            NeuronId::new("2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98")
                .unwrap();
        let mut neuron_1 = Neuron::default();
        neuron_1.id = Some(neuron_id_1.clone());

        mutate_state(|state| {
            update_neuron_maturity(state, &neuron_1);
        });

        // day 2
        neuron_1.maturity_e8s_equivalent = 100;
        neuron_1.staked_maturity_e8s_equivalent = Some(50);

        mutate_state(|state| {
            update_neuron_maturity(state, &neuron_1);
        });

        let rewarded_neurons = vec![neuron_id_1.clone()];

        let neuron_interval_maturity = vec![(neuron_id_1.clone(), 150)];

        mutate_state(|state| {
            update_neuron_reward(&rewarded_neurons, state, &neuron_interval_maturity);
        });

        read_state(|state| {
            let updated_neuron = state.data.neuron_maturity.get(&neuron_id_1).unwrap();
            assert_eq!(updated_neuron.rewarded_maturity, 150);
        })
    }
}

// async fn query_token_balance_icrc1(ledger_id: Principal) -> Result<Nat, String> {
//     info!("aa processing request");
//     let a = BalanceQuery {
//         owner: ic_cdk::api::id(),
//         subaccount: Some(DEFAULT_SUBACCOUNT), // Adjust according to your data type
//     };
//     info!("bb processing request");
//     ic_ledger_types::
//     if let Ok(res)  = call(ledger_id, "icrc1_balance_of", (a,)).await {
//         return res
//     } else {
//         Err("failed to get icp balance".to_owned())
//     }

// }
