/*!
# SNS reward distribution

This job is responsible for distributing rewards to user's sub accounts.

There are reward pools ( ICP, OGY, GLDGov ) that exist on the 0 sub account
Individual neuron rewards are transferred to a sub account based on the NeuronId

TODO - update this.
*/

use crate::{
    model::payment_processor::{
        MaturityDelta,
        Payment,
        PaymentRound,
        PaymentRoundStatus,
        PaymentStatus,
    },
    state::{ mutate_state, read_state, RuntimeState },
};
use candid::{ Nat, Principal };
use canister_time::{ run_interval, DAY_IN_MS, WEEK_IN_MS };
use futures::{ future::{ err, join_all }, Future };
use ic_ledger_types::{ Subaccount, DEFAULT_SUBACCOUNT };
use icrc_ledger_types::icrc1::{ account::Account, transfer::TransferArg };
use num_bigint::BigUint;
use sns_governance_canister::types::NeuronId;
use std::collections::BTreeMap;
use std::time::Duration;
use tracing::{ debug, error, info };
use types::{ Milliseconds, NeuronInfo, TokenSymbol };
use utils::consts::E8S_PER_ICP;

const DISTRIBUTION_INTERVAL: Milliseconds = WEEK_IN_MS;
const DISTRIBUTION_RETRY_INTERVAL: Milliseconds = DAY_IN_MS;

pub fn start_job() {
    run_interval(Duration::from_millis(DISTRIBUTION_INTERVAL), run_distribution);
    run_interval(Duration::from_millis(DISTRIBUTION_RETRY_INTERVAL), run_retry_distribution);
}

pub fn run_distribution() {
    ic_cdk::spawn(distribute_rewards())
}

pub fn run_retry_distribution() {
    ic_cdk::spawn(retry_faulty_payment_rounds())
}

// called once per day
pub async fn retry_faulty_payment_rounds() {
    let faulty_payment_rounds = read_state(|state|
        state.data.payment_processor.get_active_faulty_payment_rounds()
    );
    if faulty_payment_rounds.len() == 0 {
        return;
    }
    for payment_round in &faulty_payment_rounds {
        process_payment_round(payment_round).await;
    }

    // update round status
    let processed_payment_rounds = read_state(|state|
        state.data.payment_processor.get_active_rounds()
    );
    for (_, payment_round) in &processed_payment_rounds {
        update_payment_round_status(&payment_round);
    }

    // post processing
    let processed_payment_rounds = read_state(|state|
        state.data.payment_processor.get_active_rounds()
    );
    for (_, payment_round) in &processed_payment_rounds {
        update_neuron_rewards(&payment_round);
        move_payment_round_to_history(&payment_round);
        log_payment_round_metrics(&payment_round);
    }
    debug!("END - finished processing distribution of payment rounds");
}

pub async fn distribute_rewards() {
    let contains_faulty_payment_rounds = read_state(|state|
        state.data.payment_processor.active_rounds_exist()
    );

    if contains_faulty_payment_rounds {
        info!("There are still active rounds present to process");
        return;
    }

    // create a new payment round
    // let reward_tokens = vec![TokenSymbol::ICP, TokenSymbol::OGY, TokenSymbol::GLDGov]; // TODO - uncomment when going live
    let reward_tokens = vec![TokenSymbol::ICP];
    for token in &reward_tokens {
        debug!("Creating new payment round for token : {:?}", token);
        // check reward pool has a balance
        let ledger_id = read_state(|state| get_ledger_id(state, token.clone()));
        // let tokens_to_distribute = fetch_reward_pool_balance(ledger_id).await;
        let tokens_to_distribute = Nat::from(300_000u64);
        if tokens_to_distribute == Nat::from(0u64) {
            info!("REWARD POOL for {:?} token has no rewards for distribution", token);
            continue;
        }

        let neuron_maturity_for_interval = read_state(|state|
            calculate_neuron_maturity_for_interval(&state.data.neuron_maturity, &token)
        );

        let transaction_fees = calculate_transaction_fees(&neuron_maturity_for_interval);
        if transaction_fees > tokens_to_distribute {
            info!(
                "The fees exceed the amount in the reward pool for token : {:?} - skipping distribution for this token",
                token.clone()
            );
            continue;
        }
        let tokens_to_distribute = tokens_to_distribute - transaction_fees;

        let total_neuron_maturity_for_interval = calculate_aggregated_maturity(
            &neuron_maturity_for_interval
        );

        if total_neuron_maturity_for_interval == 0u64 {
            info!(
                "Maturity for all neurons has not changed since last distribution - exiting distribution early"
            );
            return;
        }

        // rewards per neuron
        let neuron_share = calculate_neuron_shares(
            neuron_maturity_for_interval,
            tokens_to_distribute.clone()
        ).unwrap_or(BTreeMap::new());

        let new_round_key = read_state(|state| state.data.payment_processor.next_key());

        let new_round = PaymentRound::new(
            new_round_key,
            tokens_to_distribute,
            ledger_id,
            token.clone(),
            total_neuron_maturity_for_interval,
            neuron_share
        );
        let res = transfer_funds_to_payment_round_account(&new_round).await;
        match res {
            Ok(()) => {
                mutate_state(|state| {
                    state.data.payment_processor.add_active_payment_round(new_round);
                });
            }
            Err(e) => {
                debug!("ERROR - transferring funds to payment round sub account : {}", e);
            }
        }
    }

    // process active rounds
    let pending_payment_rounds = read_state(|state|
        state.data.payment_processor.read_active_pending_payment_rounds()
    );
    for payment_round in &pending_payment_rounds {
        process_payment_round(payment_round).await;
    }

    // update round status
    let processed_payment_rounds = read_state(|state|
        state.data.payment_processor.get_active_rounds()
    );
    for (_, payment_round) in &processed_payment_rounds {
        update_payment_round_status(&payment_round);
    }

    // post processing
    let processed_payment_rounds = read_state(|state|
        state.data.payment_processor.get_active_rounds()
    );
    for (_, payment_round) in &processed_payment_rounds {
        update_neuron_rewards(&payment_round);
        move_payment_round_to_history(&payment_round);
        log_payment_round_metrics(&payment_round);
    }
    info!("END - finished processing distribution of payment rounds");
}

pub fn move_payment_round_to_history(payment_round: &PaymentRound) {
    let payment_round_id = payment_round.id;
    let status = payment_round.round_status.clone();

    // only payment rounds that are fully completed may move to history
    if status != PaymentRoundStatus::CompletedFull {
        return;
    }

    // insert to history
    mutate_state(|state| state.data.payment_processor.add_to_history(payment_round.clone()));
    // delete from active
    mutate_state(|state| state.data.payment_processor.delete_active_round(payment_round_id));
}

pub fn log_payment_round_metrics(payment_round: &PaymentRound) -> String {
    let payments: Vec<(&NeuronId, &Payment)> = payment_round.payments.iter().collect();

    let successful_neuron_transfers: Vec<(&NeuronId, &MaturityDelta, &TokenSymbol)> = payments
        .iter()
        .filter(|(_, (_, status, _))| status == &PaymentStatus::Completed)
        .map(|(neuron_id, (_, _, maturity))| (*neuron_id, maturity, &payment_round.token))
        .collect();
    let total_successful: u64 = successful_neuron_transfers
        .iter()
        .map(|(_, maturity_delta, _)| *maturity_delta)
        .sum();
    let total_transfers = &payments.len();

    let print_string = format!(
        "PAYMENT ROUND METRICS || round id : {}, token : {:?}, total : {}, successful : {}, maturity distributed : {}, round maturity : {}",
        payment_round.id,
        payment_round.token,
        total_transfers,
        successful_neuron_transfers.len(),
        total_successful,
        payment_round.total_neuron_maturity
    );
    info!(print_string);
    print_string
}

pub async fn transfer_funds_to_payment_round_account(round: &PaymentRound) -> Result<(), String> {
    let next_key = round.id;
    let funds = round.round_funds_total.clone();
    let ledger_id = round.ledger_id.clone();
    let round_pool_subaccount = round.get_payment_round_sub_account_id();

    let from_sub_account = Subaccount([0; 32]);
    let account = Account {
        owner: ic_cdk::api::id(),
        subaccount: Some(round_pool_subaccount.0),
    };

    info!("Transferring funds to payment round sub account for round id : {}", next_key);
    transfer_token(from_sub_account, account, ledger_id, funds).await
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
) -> Option<BTreeMap<NeuronId, Payment>> {
    let total_maturity: u64 = neuron_deltas
        .iter()
        .map(|entry| entry.1)
        .sum();

    let total_maturity_big = BigUint::from(total_maturity.clone());

    if total_maturity_big == BigUint::from(0u64) {
        // if we don't return early then a dividing error will occur
        return None;
    }
    let reward_pool_big = BigUint::from(reward_pool);
    // Calculate the reward for each neuron
    let map: BTreeMap<NeuronId, Payment> = neuron_deltas
        .iter()
        .map(|(neuron_id, maturity)| {
            // Convert maturity to BigUint
            let maturity_big = BigUint::from(*maturity);

            // Calculate percentage as (maturity / total_maturity) * 10000 (expressed in basis points)
            let percentage =
                (maturity_big * BigUint::from(E8S_PER_ICP)) / total_maturity_big.clone();

            let reward = (reward_pool_big.clone() * percentage) / BigUint::from(E8S_PER_ICP);
            let reward: u64 = reward.try_into().expect("failed to convert bigint to u64");
            (neuron_id.clone(), (reward, PaymentStatus::Pending, maturity.clone()))
        })
        .filter(|(_, (reward, _, _))| reward.clone() > 0u64)
        .collect();

    Some(map)
}

pub fn update_neuron_rewards(payment_round: &PaymentRound) {
    let payments: Vec<(&NeuronId, &Payment)> = payment_round.payments.iter().collect();

    let successful_neuron_transfers: Vec<(&NeuronId, &MaturityDelta, &TokenSymbol)> = payments
        .iter()
        .filter(|(_, (_, status, _))| status == &PaymentStatus::Completed)
        .map(|(neuron_id, (_, _, maturity))| (*neuron_id, maturity, &payment_round.token))
        .collect();

    // println!("/// successful_neuron_transfers {:?}", successful_neuron_transfers);
    for (neuron_id, maturity_delta, token) in successful_neuron_transfers {
        mutate_state(|state| {
            if let Some(neuron) = state.data.neuron_maturity.get_mut(&neuron_id) {
                if let Some(rewarded_maturity) = neuron.rewarded_maturity.get_mut(&token) {
                    let new_maturity = rewarded_maturity
                        .checked_add(*maturity_delta)
                        .expect("update_neuron_rewards - overflow");
                    *rewarded_maturity = new_maturity;
                } else {
                    neuron.rewarded_maturity.insert(*token, *maturity_delta);
                }
            }
        });
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

async fn transfer_token(
    from_sub_account: Subaccount,
    to_account: Account,
    ledger_id: Principal,
    amount: Nat
) -> Result<(), String> {
    match
        icrc_ledger_canister_c2c_client::icrc1_transfer(
            ledger_id,
            &(TransferArg {
                from_subaccount: Some(from_sub_account.0),
                to: to_account,
                fee: Some((10_000u32).into()),
                created_at_time: None,
                amount: amount,
                memo: None,
            })
        ).await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(error)) => Err(format!("Transfer error: {error:?}")),
        Err(error) => Err(format!("Network error: {error:?}")),
    }
}

fn always_fail_future() -> impl Future<Output = Result<(), String>> {
    // Create and return a future that always returns an Err
    err("simulated failure".to_string())
}

fn update_payment_round_status(payment_round: &PaymentRound) -> PaymentRoundStatus {
    let payments: Vec<(&NeuronId, &Payment)> = payment_round.payments.iter().collect();

    let mut completed_count = 0;
    let mut failed_count = 0;

    for (_, (_, payment_status, _)) in &payments {
        match payment_status {
            PaymentStatus::Completed => {
                completed_count += 1;
            }
            PaymentStatus::Failed(_) => {
                failed_count += 1;
            }
            _ => {} // Ignore other statuses
        }
    }
    let new_status: PaymentRoundStatus;
    if completed_count > 0 && failed_count > 0 {
        new_status = PaymentRoundStatus::CompletedPartial;
    } else if completed_count == payments.len() {
        new_status = PaymentRoundStatus::CompletedFull;
    } else {
        new_status = PaymentRoundStatus::Failed(
            "All payments for payment round failed".to_string()
        );
    }
    mutate_state(|state|
        state.data.payment_processor.set_active_round_status(&payment_round.id, new_status.clone())
    );
    new_status
}

pub async fn process_payment_round((round_id, payment_round): &(u16, PaymentRound)) {
    debug!("START - payment processing of {:?} for round id : {}", payment_round.token, round_id);
    let batch_limit = 45;
    let round_pool_subaccount = payment_round.get_payment_round_sub_account_id();
    let ledger_id = payment_round.ledger_id;
    mutate_state(|state| {
        state.data.payment_processor.set_active_round_status(
            &round_id,
            PaymentRoundStatus::InProgress
        );
    });

    let payments: Vec<(&NeuronId, &Payment)> = payment_round.payments
        .iter()
        .filter(|(_, (_, payment_status, _))| payment_status != &PaymentStatus::Completed)
        .collect();
    let mut payment_chunks = payments.chunks(batch_limit);

    while let Some(batch) = payment_chunks.next() {
        let (transfer_futures, neuron_ids): (Vec<_>, Vec<_>) = batch
            .iter()
            .map(|(neuron_id, (reward, _, _))| {
                let n_id = *neuron_id;
                let account = Account {
                    owner: ic_cdk::api::id(),
                    subaccount: Some(n_id.into()),
                };
                mutate_state(|state|
                    state.data.payment_processor.set_active_payment_status(
                        &round_id,
                        &neuron_id,
                        PaymentStatus::Triggered
                    )
                );
                let transfer_future = transfer_token(
                    round_pool_subaccount,
                    account,
                    ledger_id,
                    Nat::from(*reward)
                );
                (transfer_future, *neuron_id) //
                // (always_fail_future(), *neuron_id)
            })
            .unzip();

        let results = join_all(transfer_futures).await;

        for (result, neuron_id) in results.into_iter().zip(neuron_ids.into_iter()) {
            match result {
                Ok(_) => {
                    mutate_state(|state|
                        state.data.payment_processor.set_active_payment_status(
                            &round_id,
                            &neuron_id,
                            PaymentStatus::Completed
                        )
                    );
                }
                Err(e) => {
                    mutate_state(|state|
                        state.data.payment_processor.set_active_payment_status(
                            &round_id,
                            &neuron_id,
                            PaymentStatus::Failed(e.clone())
                        )
                    );
                }
            }
        }
    }
}

pub fn calculate_transaction_fees(neuron_maturity_deltas: &Vec<(NeuronId, u64)>) -> Nat {
    let neurons_with_positive_maturity_delta: Vec<&(NeuronId, u64)> = neuron_maturity_deltas
        .iter()
        .filter(|(_, maturity)| *maturity > 0u64)
        .collect();

    let single_fee = 10_000u64; // TODO - is this the same for gldgov and ogy
    let number_of_valid_transactions = neurons_with_positive_maturity_delta.len() as u64;
    let total_fees = number_of_valid_transactions
        .checked_mul(single_fee)
        .expect("overflow when calculating total fees");
    Nat::from(total_fees)
}

#[cfg(test)]
mod tests {
    use std::collections::{ BTreeMap, HashMap };

    use candid::{ Nat, Principal };
    use sns_governance_canister::types::NeuronId;
    use types::{ NeuronInfo, TokenSymbol };

    use crate::{
        jobs::distribute_rewards::calculate_neuron_shares,
        model::payment_processor::{ PaymentRound, PaymentStatus },
        state::{ init_state, mutate_state, read_state, RuntimeState },
    };

    use super::{
        calculate_aggregated_maturity,
        calculate_neuron_maturity_for_interval,
        calculate_transaction_fees,
        log_payment_round_metrics,
        update_neuron_rewards,
    };

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    #[test]
    fn test_calculate_neuron_maturity_for_first_sync() {}

    #[test]
    fn test_calculate_neuron_shares() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let neuron_deltas = vec![(neuron_id_1, 10u64), (neuron_id_2, 20u64), (neuron_id_3, 30u64)];
        let reward_pool = Nat::from(100_000_000u64); // 1 ICP
        let expected: Vec<u64> = vec![16_666_666u64, 33_333_333u64, 50_000_000u64];

        let result = calculate_neuron_shares(neuron_deltas, reward_pool).unwrap();
        result
            .iter()
            .zip(expected.iter())
            .for_each(|(res, expected_value)| {
                assert_eq!(&res.1.0, expected_value);
            });
    }
    #[test]
    fn test_calculate_neuron_shares_all_zeros() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let neuron_deltas = vec![(neuron_id_1, 0u64), (neuron_id_2, 0u64), (neuron_id_3, 0u64)];
        let reward_pool = Nat::from(100_000_000u64); // 1 ICP

        let result = calculate_neuron_shares(neuron_deltas, reward_pool).is_none();
        assert_eq!(result, true)
    }

    #[test]
    fn test_calculate_neuron_shares_with_no_maturity_change() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let neuron_deltas = vec![(neuron_id_1, 0u64), (neuron_id_2, 30u64), (neuron_id_3, 30u64)];
        let reward_pool = Nat::from(100_000_000u64); // 1 ICP
        let expected: Vec<u64> = vec![50_000_000u64, 50_000_000u64];

        let result = calculate_neuron_shares(neuron_deltas, reward_pool).unwrap();
        result
            .iter()
            .zip(expected.iter())
            .for_each(|(res, expected_value)| {
                assert_eq!(&res.1.0, expected_value);
            });
    }

    #[test]
    fn test_calculate_neuron_maturity_for_interval() {
        let mut neurons = BTreeMap::new();

        // neuron 1
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let mut neuron_1_rewarded = HashMap::new();
        neuron_1_rewarded.insert(TokenSymbol::ICP, 0);

        let neuron_info_1 = NeuronInfo {
            accumulated_maturity: 150,
            last_synced_maturity: 150,
            rewarded_maturity: neuron_1_rewarded,
        };
        neurons.insert(neuron_id_1.clone(), neuron_info_1);

        let result = calculate_neuron_maturity_for_interval(&neurons, &TokenSymbol::ICP);
        let expected = 150;
        assert_eq!(result[0].1, expected);

        // simulate paying the user

        // payout previous maturity ( 150 ) && update the neuron maturity ( simulate new neuron maturity data )
        let n = neurons.get_mut(&neuron_id_1).unwrap();
        n.accumulated_maturity = 542;
        n.last_synced_maturity = 542;
        let rewarded_mat = n.rewarded_maturity.get_mut(&TokenSymbol::ICP).unwrap();
        *rewarded_mat += 150;

        let result = calculate_neuron_maturity_for_interval(&neurons, &TokenSymbol::ICP);
        println!("{:?}", neurons);
        let expected = 392; // 542 (current maturity) - 150 (previous maturity)
        assert_eq!(result[0].1, expected);
    }

    #[test]
    fn test_calculate_neuron_maturity_for_interval_all_zeros() {
        let mut neurons = BTreeMap::new();

        // neuron 1
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let mut neuron_1_rewarded = HashMap::new();
        neuron_1_rewarded.insert(TokenSymbol::ICP, 0);

        let neuron_info_1 = NeuronInfo {
            accumulated_maturity: 0,
            last_synced_maturity: 0,
            rewarded_maturity: neuron_1_rewarded,
        };
        neurons.insert(neuron_id_1.clone(), neuron_info_1);

        let result = calculate_neuron_maturity_for_interval(&neurons, &TokenSymbol::ICP);
        let expected = 0;
        assert_eq!(result[0].1, expected);
    }

    #[test]
    fn test_calculate_aggregated_maturity() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let neuron_deltas = vec![(neuron_id_1, 10u64), (neuron_id_2, 20u64), (neuron_id_3, 30u64)];
        let res = calculate_aggregated_maturity(&neuron_deltas);
        let expected = 60u64;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_log_payment_round_metrics() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_4 = NeuronId::new(
            "5a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_5 = NeuronId::new(
            "6a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let mut payments = BTreeMap::new();
        payments.insert(neuron_id_1, (1, PaymentStatus::Failed("simulated fail".to_string()), 1));
        payments.insert(neuron_id_2, (1, PaymentStatus::Completed, 1));
        payments.insert(neuron_id_3, (1, PaymentStatus::Completed, 1));
        payments.insert(neuron_id_4, (1, PaymentStatus::Completed, 1));
        payments.insert(neuron_id_5, (1, PaymentStatus::Completed, 1));

        let ledger_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

        let round = PaymentRound::new(
            1u16,
            Nat::from(100_000u64),
            ledger_id,
            TokenSymbol::ICP,
            5u64,
            payments
        );

        let result = log_payment_round_metrics(&round);

        assert_eq!(
            result,
            "PAYMENT ROUND METRICS || round id : 1, token : ICP, total : 5, successful : 4, maturity distributed : 4, round maturity : 5"
        );
    }

    #[test]
    fn test_update_neuron_rewards() {
        init_runtime_state();

        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        // insert a neuron to neuron_maturity
        let expected_result = 150u64;

        let mut neuron_1_rewarded = HashMap::new();
        neuron_1_rewarded.insert(TokenSymbol::ICP, 0);

        let neuron_info = NeuronInfo {
            accumulated_maturity: 0,
            last_synced_maturity: 0,
            rewarded_maturity: neuron_1_rewarded,
        };

        mutate_state(|state| {
            state.data.neuron_maturity.insert(neuron_id_1.clone(), neuron_info);
        });

        // create a payment round

        let mut payments = BTreeMap::new();
        payments.insert(neuron_id_1.clone(), (1, PaymentStatus::Completed, expected_result));

        let ledger_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

        let round = PaymentRound::new(
            1u16,
            Nat::from(100_000u64),
            ledger_id,
            TokenSymbol::ICP,
            10u64,
            payments
        );

        update_neuron_rewards(&round);

        // test 1
        read_state(|state| {
            let neuron = state.data.neuron_maturity.get(&neuron_id_1).unwrap();
            let rewarded_amount = neuron.rewarded_maturity.get(&TokenSymbol::ICP).unwrap();
            assert_eq!(rewarded_amount.clone(), expected_result);
        });

        // dont strictly need to do this
        mutate_state(|state| {
            let neuron_maturity = state.data.neuron_maturity.get_mut(&neuron_id_1).unwrap();
            neuron_maturity.accumulated_maturity += 150; // 450 in total now
        });
        // use same payment round from before
        update_neuron_rewards(&round);
        let expected_result = 300u64; // two payments of 150

        read_state(|state| {
            let neuron = state.data.neuron_maturity.get(&neuron_id_1).unwrap();
            let rewarded_amount = neuron.rewarded_maturity.get(&TokenSymbol::ICP).unwrap();
            assert_eq!(rewarded_amount.clone(), expected_result);
        });

        // update the neuron maturity
    }

    #[test]
    fn test_calculate_transaction_fees() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let neuron_deltas = vec![(neuron_id_1, 0u64), (neuron_id_2, 30u64), (neuron_id_3, 30u64)];
        let expected = Nat::from(20_000u64); // 2 x neurons with positive maturity

        let result = calculate_transaction_fees(&neuron_deltas);
        assert_eq!(result, expected);
    }
}
