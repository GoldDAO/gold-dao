/*!
# SNS reward distribution

This job is responsible for distributing rewards to user's sub accounts.

There are reward pools ( ICP, OGY, GLDGov ) that exist on the 0 sub account
Individual neuron rewards are transferred to a sub account based on the NeuronId

TODO - update this.
*/

use crate::{
    model::payment_processor::{ MaturityDelta, Payment, PaymentRound, PaymentStatus },
    state::{ mutate_state, read_state, RuntimeState },
};
use candid::{ Nat, Principal };
use canister_time::{ run_interval, WEEK_IN_MS };
use ic_ledger_types::DEFAULT_SUBACCOUNT;
use icrc_ledger_types::icrc1::account::Account;
use num_bigint::BigUint;
use sns_governance_canister::types::NeuronId;
use std::collections::{ BTreeMap, HashMap };
use std::time::Duration;
use tracing::{ debug, error, info };
use types::{ Milliseconds, NeuronInfo, TokenSymbol };
use utils::consts::E8S_PER_ICP;

const DISTRIBUTION_INTERVAL: Milliseconds = WEEK_IN_MS;

pub fn start_job() {
    run_interval(Duration::from_millis(DISTRIBUTION_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(distribute_rewards())
}

// called once per day
pub async fn retry_faulty_payment_rounds() {
    let contains_faulty_payment_rounds = read_state(|state|
        state.data.payment_processor.contains_faulty_payment_rounds()
    );
    if !contains_faulty_payment_rounds {
        info!("All payment rounds are COMPLETED or PENDING");
        return;
    }
    let successful_neuron_payments = mutate_state(|state| {
        state.data.payment_processor.process_faulty_rounds()
    });

    mutate_state(|state| {
        update_neuron_rewards(state, successful_neuron_payments);
    });
    // TODO
    // Add the job duration etc
}

// called once per week
pub async fn distribute_rewards() {
    // only create a new payment run if all previous are COMPLETED
    let contains_faulty_payment_rounds = read_state(|state|
        state.data.payment_processor.contains_faulty_payment_rounds()
    );

    if contains_faulty_payment_rounds {
        info!(
            "failed to rerun previous faulty payment rounds. will not create new payment rounds until previous are fixed"
        );
    }
    // create a new payment run
    // let reward_tokens = vec![TokenSymbol::ICP, TokenSymbol::OGY, TokenSymbol::GLDGov];
    let reward_tokens = vec![TokenSymbol::ICP];
    for token in &reward_tokens {
        debug!("Creating new payment round for token : {:?}", token);
        // check reward pool has a balance
        let ledger_id = read_state(|state| get_ledger_id(state, token.clone()));
        let tokens_to_distribute = fetch_reward_pool_balance(ledger_id).await;
        if tokens_to_distribute == Nat::from(0u64) {
            return;
        }
        debug!("Tokens to distribute {}", tokens_to_distribute);
        // maturity delta per neuron
        let neuron_maturity_for_interval = read_state(|state|
            calculate_neuron_maturity_for_interval(&state.data.neuron_maturity, &token)
        );

        // total neuron_maturity
        let total_neuron_maturity_for_interval = calculate_aggregated_maturity(
            &neuron_maturity_for_interval
        );

        // rewards per neuron
        let neuron_share = calculate_neuron_shares(
            neuron_maturity_for_interval,
            tokens_to_distribute.clone()
        );

        // TODO create a payment round
        // e.g state.payment_rounds.add_payment_round(neuron_share, token)
        mutate_state(|state| {
            let new_round = PaymentRound::new(
                tokens_to_distribute,
                ledger_id,
                token.clone(),
                total_neuron_maturity_for_interval,
                neuron_share
            );
            debug!("New payment round created for token {:?}", new_round.token);

            state.data.payment_processor.add_payment_round(new_round);
        });
    }

    // let successful_neuron_payments = mutate_state(|state| {
    //     state.data.payment_processor.process_pending_payment_rounds()
    // });

    // for token in reward_tokens {
    //     let metrics: Vec<u64> = successful_neuron_payments
    //         .iter()
    //         .filter(|payment| payment.2 == token)
    //         .map(|payment| payment.1)
    //         .collect();
    //     let total_mat: u64 = metrics.iter().sum();
    //     debug!(
    //         "METRICS || token : {:?}, number success completed : {}, total_maturity distributed : {}",
    //         token,
    //         metrics.len(),
    //         total_mat
    //     );
    // }

    // mutate_state(|state| {
    //     update_neuron_rewards(state, successful_neuron_payments);
    // });
}

pub fn get_ledger_id(state: &RuntimeState, token: TokenSymbol) -> Principal {
    match token {
        TokenSymbol::ICP => state.data.icp_ledger_canister_id,
        TokenSymbol::OGY => state.data.ogy_ledger_canister_id,
        TokenSymbol::GLDGov => state.data.gldgov_ledger_canister_id,
    }
}

pub fn calculate_neuron_maturity_for_interval(
    neurons: &BTreeMap<NeuronId, NeuronInfo>,
    token: &TokenSymbol
) -> Vec<(NeuronId, u64)> {
    neurons
        .into_iter()
        .map(|(neuron_id, neuron_info)| {
            let previous_rewarded = neuron_info.rewarded_maturity
                .get(token)
                .unwrap_or(&0u64)
                .clone();
            let accumulated = neuron_info.accumulated_maturity;
            let delta_maturity = accumulated
                .checked_sub(previous_rewarded)
                .expect("overflow calculating maturity delta");
            (neuron_id.clone(), delta_maturity)
        })
        .collect()
}

pub fn calculate_neuron_shares(
    neuron_deltas: Vec<(NeuronId, u64)>,
    reward_pool: Nat
) -> HashMap<NeuronId, Payment> {
    let total_maturity: u64 = neuron_deltas
        .iter()
        .map(|entry| entry.1)
        .sum();

    let total_maturity_big = BigUint::try_from(total_maturity.clone()).unwrap();
    let reward_pool_big = BigUint::from(reward_pool);
    // Calculate the reward for each neuron
    let map: HashMap<NeuronId, Payment> = neuron_deltas
        .iter()
        .map(|(neuron_id, maturity)| {
            // Convert maturity to BigUint
            let maturity_big = BigUint::try_from(*maturity).unwrap();

            // Calculate percentage as (maturity / total_maturity) * 10000 (expressed in basis points)
            let percentage =
                (maturity_big * BigUint::from(E8S_PER_ICP)) / total_maturity_big.clone();

            let reward = (reward_pool_big.clone() * percentage) / BigUint::from(E8S_PER_ICP);
            let reward: u64 = reward.try_into().expect("failed to convert bigint to u64");
            (neuron_id.clone(), (reward, PaymentStatus::Pending, maturity.clone()))
        })
        .collect();

    map
}

pub fn update_neuron_rewards(
    state: &mut RuntimeState,
    successful_neuron_transfers: Vec<(NeuronId, MaturityDelta, TokenSymbol)>
) {
    for (neuron_id, maturity_delta, token) in successful_neuron_transfers {
        let neuron = state.data.neuron_maturity.get_mut(&neuron_id);
        match neuron {
            Some(neuron_info) => {
                let rewarded_maturity_token = neuron_info.rewarded_maturity.get_mut(&token);
                match rewarded_maturity_token {
                    Some(value) => {
                        value
                            .checked_add(maturity_delta)
                            .expect(
                                "update_neuron_rewards - overflow when adding neuron maturity to existing maturity"
                            );
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}

pub fn calculate_aggregated_maturity(data: &Vec<(NeuronId, u64)>) -> u64 {
    data.iter()
        .map(|entry| entry.1)
        .sum()
}

async fn fetch_reward_pool_balance(ledger_canister_id: Principal) -> Nat {
    match
        icrc_ledger_canister_c2c_client::icrc1_balance_of(
            ledger_canister_id,
            &(Account {
                owner: ic_cdk::api::id(),
                subaccount: Some(DEFAULT_SUBACCOUNT.0),
            })
        ).await
    {
        Ok(t) => {
            info!("Success - querying balance of {} - has {}", ledger_canister_id, t);
            t
        }
        Err(e) => {
            error!(
                "Fail - to fetch token balance of ledger canister id {ledger_canister_id} with ERROR_CODE : {} . MESSAGE",
                e.1
            );
            Nat::from(0u64)
        }
    }
}

#[cfg(test)]
mod tests {
    // use num_bigint::BigUint;
    // use sns_governance_canister::types::NeuronId;
    // use types::NeuronInfo;
    // use utils::consts::E8S_PER_ICP;

    // use crate::{
    //     jobs::distribute_rewards::{
    //         calculate_aggregated_maturity,
    //         calculate_neuron_percentages,
    //         calculate_reward,
    //         update_neuron_reward,
    //     },
    //     state::{ init_state, mutate_state, read_state, RuntimeState },
    // };

    // use super::calculate_neuron_maturity_for_interval;

    #[test]
    fn test_calculate_neuron_maturity_for_first_sync() {}
}
