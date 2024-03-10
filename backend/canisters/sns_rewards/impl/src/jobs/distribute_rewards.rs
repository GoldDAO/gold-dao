/*!
# SNS reward distribution

This job is responsible for distributing rewards to user's sub accounts.
All the different reward tokens are to be held in the 0 sub account.
*/

use crate::state::{mutate_state, read_state};
use candid::{CandidType, Principal};
use canister_time::{now_millis, run_interval, WEEK_IN_MS};
use ic_cdk::api::call::call;
use ic_ledger_types::Subaccount;
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use std::time::Duration;
use std::{
    collections::BTreeMap,
    ops::Mul,
};
use tracing::{debug, error, info, warn};
use types::{Milliseconds, NeuronInfo};
use utils::consts::ICP_LEDGER_CANISTER_ID;
use futures::{future::try_join_all, try_join};


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

    // 1 ) Cacluating neuron reward percentage
    let neuron_maturity_for_interval = read_state(|state| {
        calculate_neuron_maturity_for_interval(
            &state.data.neuron_maturity,
        )
    });
    let total_maturity_for_all_neurons =
        calculate_aggregated_maturity(&neuron_maturity_for_interval);
    let neuron_reward_percentage = calculate_neuron_reward_percentages(
        &neuron_maturity_for_interval,
        &total_maturity_for_all_neurons,
    );

    // 2 ) Get balances of all reward pools
    // let ogy_reward_pool_id = read_state(|state| state.data.ogy_ledger_canister);
    // let ogy_reward_pool_balance = query_token_balance_icrc1(ogy_reward_pool_id).await;

    let icp_reward_pool_balance = query_token_balance_icrc1(ICP_LEDGER_CANISTER_ID)
        .await
        .expect("failed to get balance of sns_rewards ICP balance");

    // 4 ) Pay all sub accounts
    let sucessful_neuron_transfers = transfer_rewards(neuron_reward_percentage, icp_reward_pool_balance).await;

    // update the neuron info with the amount of maturity paid
    mutate_state(|state| {
        sucessful_neuron_transfers.iter().for_each(|neuron_id| {
            let neuron = state.data.neuron_maturity.get_mut(neuron_id);

            match neuron {
                Some(neuron_info) => {
                    let neuron_maturity = neuron_maturity_for_interval.iter().find(|(n_id, _)| n_id == neuron_id).unwrap();
                    neuron_info.rewarded_maturity += neuron_maturity.1;
                },
                None => {}
            }

        })
    })

}

fn f64_to_e8(value: f64) -> u64 {
    (value * 1_000_000_00.0) as u64
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

pub fn calculate_neuron_reward_percentages(
    data: &Vec<(NeuronId, u64)>,
    total_maturity: &u64,
) -> Vec<(NeuronId, f64)> {
    data.iter()
        .map(|entry| {
            let percentage = entry.1 as f64 / total_maturity.clone() as f64;
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

async fn transfer_icp_to_sub_account(sub_account: Subaccount, amount: u64) -> Result<u64, String> {
    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        ICP_LEDGER_CANISTER_ID,
        &(TransferArg {
            from_subaccount: None,
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
        Ok(Ok(_)) => Ok(1),
        Ok(Err(error)) => {
            return Err(format!("Transfer error: {error:?}"));
        }
        Err(error) => {
            return Err(format!("Network error: {error:?}"));
        }
    }
}

async fn transfer_rewards(neurons: Vec<(NeuronId, f64)>, icp_balance: u64) -> Vec<NeuronId> {
    let mut successful_reward_transfers : Vec<NeuronId> = vec![];
    neurons
        .iter()
        .for_each(|(neuron_id, percentage_to_reward)| {
            let _ = async {
                if percentage_to_reward.clone() <= 0.0f64 {
                    return; // skip payment since this neuron has no percentage
                }

                let mut all_successful = true;

                let mut transfer_futures = Vec::new();


                let neuron_id_as_bytes = neuron_id
                    .clone()
                    .into_array()
                    .expect("Error conerting NeuronId into u8");
                let sub_account = Subaccount(neuron_id_as_bytes);

                // handle ICP transfer
                let icp_reward = (icp_balance as f64).mul(percentage_to_reward);
                let icp_reward = f64_to_e8(icp_reward);
                let icp_transfer = transfer_icp_to_sub_account(sub_account, icp_reward);

                transfer_futures.push(async {
                    match try_join!(icp_transfer) {
                        Ok(_) => Ok(()),
                        Err(err) => {
                            error!(err);
                            Err(())
                        }
                    }
                });

                let results = futures::executor::block_on(async {
                    try_join_all(transfer_futures).await
                });
                
                if let Err(_) = results {
                    all_successful = false;
                }
                
                if all_successful {
                    successful_reward_transfers.push(neuron_id.clone());
                } else {
                    warn!("failed to transfer rewards for neuron : {}", neuron_id);
                }
               
            };
        });
        successful_reward_transfers
}

#[cfg(test)]
mod tests {
    use sns_governance_canister::types::{Neuron, NeuronId};

    use crate::{
        jobs::{
            distribute_rewards::calculate_aggregated_maturity,
            distribute_rewards::calculate_neuron_reward_percentages,
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
            let d = calculate_neuron_maturity_for_interval(
                &state.data.neuron_maturity,
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
            );
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
            let d = calculate_neuron_maturity_for_interval(
                &state.data.neuron_maturity,
            );
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
            calculate_neuron_reward_percentages(&neuron_list, &total_maturity);
        let expected_vals: Vec<f64> = vec![0.16666666666666666, 0.33333333333333333, 0.50];

        let test_run: Vec<((NeuronId, f64), f64)> = maturity_percentage_per_neuron
            .iter()
            .zip(expected_vals.iter())
            .map(|((neuron_id, percentage), expected_val)| {
                ((neuron_id.clone(), *percentage), *expected_val)
            })
            .collect();

        for ((_, percentage), expected_val) in test_run {
            assert_eq!(percentage, expected_val);
        }
    }
}
