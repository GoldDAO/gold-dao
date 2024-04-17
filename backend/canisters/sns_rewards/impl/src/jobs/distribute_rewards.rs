/*!
# SNS reward distribution

- fn distribute_rewards
Distributes reward tokens based on a neuron's accumulated maturity 
on a WEEKLY basis. 

- Sub accounts
reward pool - [0u8;32] -> holds ICP, OGY, GLDGov pre distribution
payment round pool - [0u8;30,u16] -> all 0's except from the last 2. represents a reward pool for a specific distribution round
neuron / user reward - [u8;32] -> based on the NeuronId ( since both are a [u8;32] )

- PaymentRounds & Payments
new PaymentRounds may only be created if there are no active payment rounds.
payments are done in batches and upon each individual transfer response it's status is updated.

*/

use crate::{
    consts::REWARD_POOL_SUB_ACCOUNT,
    model::payment_processor::{
        MaturityDelta,
        Payment,
        PaymentRound,
        PaymentRoundStatus,
        PaymentStatus,
    },
    state::{ mutate_state, read_state },
    utils::transfer_token,
};
use candid::{ Nat, Principal };
use canister_time::{ run_interval, WEEK_IN_MS };
use futures::{ future::{ err, join_all }, Future };
use icrc_ledger_types::icrc1::account::Account;
use sns_governance_canister::types::NeuronId;
use std::time::Duration;
use tracing::{ debug, error, info };
use types::{ Milliseconds, TokenSymbol };

const DISTRIBUTION_INTERVAL: Milliseconds = WEEK_IN_MS;
const MAX_RETRIES: u8 = 3;

pub fn start_job() {
    run_interval(Duration::from_millis(WEEK_IN_MS), run_distribution);
}

pub fn run_distribution() {
    let is_sync_neurons_in_progress = read_state(|s| s.get_is_synchronizing_neurons());
    if is_sync_neurons_in_progress {
        debug!(
            "REWARD_DISTRIBUTION - can't run whilst synchronise_neurons is in progress. rerunning in 3 minutes"
        );
        ic_cdk_timers::set_timer(Duration::from_secs(60 * 5), run_distribution);
    } else {
        ic_cdk::spawn(distribute_rewards(0))
    }
}

pub async fn distribute_rewards(retry_attempt: u8) {
    info!("REWARD_DISTRIBUTION - START - retry attempt : {}", retry_attempt);

    let pending_payment_rounds = read_state(|state| {
        state.data.payment_processor.get_active_rounds()
    });
    // if there are active rounds or we're retrying other rounds, do not create a new payment round
    if pending_payment_rounds.len() == 0 && retry_attempt == 0 {
        create_new_payment_rounds().await;
    }

    // process active rounds
    let active_rounds = read_state(|state| { state.data.payment_processor.get_active_rounds() });
    if active_rounds.len() == 0 {
        return;
    }
    for payment_round in active_rounds {
        process_payment_round(payment_round, retry_attempt).await;
    }

    // post processing && retry failed payment rounds
    let processed_payment_rounds = read_state(|state|
        state.data.payment_processor.get_active_rounds()
    );
    if should_retry_distribution(&processed_payment_rounds) && retry_attempt < MAX_RETRIES {
        ic_cdk::spawn(distribute_rewards(retry_attempt + 1));
    } else {
        for payment_round in &processed_payment_rounds {
            update_neuron_rewards(&payment_round);
            move_payment_round_to_history(&payment_round);
            log_payment_round_metrics(&payment_round);
        }
    }
    info!("REWARD_DISTRIBUTION - FINISH");
}

pub async fn create_new_payment_rounds() {
    let reward_tokens = read_state(|s| s.data.tokens.clone());
    let new_round_key = read_state(|state| state.data.payment_processor.next_key());
    let is_test_mode = read_state(|s| s.env.is_test_mode());

    for (token, token_info) in reward_tokens.into_iter() {
        let mut reward_pool_balance = fetch_reward_pool_balance(token_info.ledger_id).await;

        // if is_test_mode {
        //     if token == TokenSymbol::parse("ICP").unwrap() {
        //         reward_pool_balance = Nat::from(33_300_000_000u64);
        //     }
        // }

        if reward_pool_balance == Nat::from(0u64) {
            info!(
                "ROUND ID : {} & TOKEN :{:?} - has no rewards for distribution",
                new_round_key,
                token
            );
            continue;
        }

        let neuron_data = read_state(|state| state.data.neuron_maturity.clone());

        let new_round = PaymentRound::new(
            new_round_key,
            reward_pool_balance,
            token_info,
            token.clone(),
            neuron_data
        );
        match new_round {
            Ok(valid_round) => {
                match transfer_funds_to_payment_round_account(&valid_round).await {
                    Ok(()) => {
                        mutate_state(|state| {
                            state.data.payment_processor.add_active_payment_round(valid_round);
                        });
                    }
                    Err(e) => {
                        debug!("ERROR - transferring funds to payment round sub account : {}", e);
                    }
                }
            }
            Err(s) => {
                info!("ROUND ID : {} & TOKEN :{:?} - Invalid round : {}", new_round_key, token, s);
                continue;
            }
        }
    }
}

pub fn should_retry_distribution(payment_rounds: &Vec<PaymentRound>) -> bool {
    let mut should_retry = false;
    for payment_round in payment_rounds {
        match determine_payment_round_status(&payment_round) {
            PaymentRoundStatus::CompletedPartial => {
                should_retry = true;
            }
            PaymentRoundStatus::Failed(_) => {
                should_retry = true;
            }
            PaymentRoundStatus::InProgress => {
                should_retry = true;
            }
            PaymentRoundStatus::Pending => {
                should_retry = true;
            }
            _ => {}
        }
    }
    should_retry
}

pub fn move_payment_round_to_history(payment_round: &PaymentRound) {
    let status = determine_payment_round_status(payment_round);

    // only payment rounds that are fully completed may move to history
    if status != PaymentRoundStatus::CompletedFull {
        return;
    }
    // insert to history && delete from active
    mutate_state(|state| state.data.payment_processor.add_to_history(payment_round.clone()));
    mutate_state(|state|
        state.data.payment_processor.delete_active_round(payment_round.token.clone())
    );
}

pub fn log_payment_round_metrics(payment_round: &PaymentRound) -> String {
    let payments: Vec<(&NeuronId, &Payment)> = payment_round.payments.iter().collect();
    let overall_status = determine_payment_round_status(payment_round);

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
        "PAYMENT ROUND METRICS || round id : {}, round status : {:?}, token : {:?}, total : {}, successful : {}, maturity distributed : {}, round maturity : {}, retries : {}, tokens_to_distribute : {}",
        payment_round.id,
        overall_status,
        payment_round.token,
        total_transfers,
        successful_neuron_transfers.len(),
        total_successful,
        payment_round.total_neuron_maturity,
        payment_round.retries,
        payment_round.tokens_to_distribute
    );
    info!(print_string);
    print_string
}

pub async fn transfer_funds_to_payment_round_account(round: &PaymentRound) -> Result<(), String> {
    let total_to_transfer = round.round_funds_total.clone();
    let ledger_id = round.ledger_id.clone();
    let round_pool_subaccount = round.get_payment_round_sub_account_id();

    let from_sub_account = REWARD_POOL_SUB_ACCOUNT;
    let account = Account {
        owner: ic_cdk::api::id(),
        subaccount: Some(round_pool_subaccount),
    };

    transfer_token(from_sub_account, account, ledger_id, total_to_transfer).await
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
                if let Some(rewarded_maturity) = neuron.rewarded_maturity.get_mut(&token.clone()) {
                    let new_maturity = rewarded_maturity.clone() + maturity_delta.clone();
                    *rewarded_maturity = new_maturity;
                } else {
                    neuron.rewarded_maturity.insert(token.clone(), *maturity_delta);
                }
            }
        });
    }
}

async fn fetch_reward_pool_balance(ledger_canister_id: Principal) -> Nat {
    match
        icrc_ledger_canister_c2c_client::icrc1_balance_of(
            ledger_canister_id,
            &(Account {
                owner: ic_cdk::api::id(),
                subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
            })
        ).await
    {
        Ok(t) => { t }
        Err(e) => {
            error!(
                "Fail - to fetch token balance of ledger canister id {ledger_canister_id} with ERROR_CODE : {} . MESSAGE",
                e.1
            );
            Nat::from(0u64)
        }
    }
}

fn determine_payment_round_status(payment_round: &PaymentRound) -> PaymentRoundStatus {
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
    new_status
}

pub async fn process_payment_round(payment_round: PaymentRound, retry_attempt: u8) {
    info!("ROUND ID : {} & TOKEN :{:?} - STARTING PAYMENTS", payment_round.id, payment_round.token);
    let batch_limit = 45;
    let round_pool_subaccount = payment_round.get_payment_round_sub_account_id();
    let ledger_id = payment_round.ledger_id;

    let payments: Vec<(&NeuronId, &Payment)> = payment_round.payments
        .iter()
        .filter(|(_, (_, payment_status, _))| payment_status != &PaymentStatus::Completed)
        .collect();
    let mut payment_chunks = payments.chunks(batch_limit);

    // update retry count
    mutate_state(|s|
        s.data.payment_processor.set_payment_round_retry_count(&payment_round.token, retry_attempt)
    );

    let total_to_process = payments.len();
    let mut processed_count = 0;

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
                        &payment_round.token,
                        &neuron_id,
                        PaymentStatus::Triggered
                    )
                );
                let transfer_future = transfer_token(
                    round_pool_subaccount,
                    account,
                    ledger_id,
                    reward.clone()
                );
                (transfer_future, *neuron_id)
                // (always_fail_future(), *neuron_id)
            })
            .unzip();

        let results = join_all(transfer_futures).await;
        for (result, neuron_id) in results.into_iter().zip(neuron_ids.into_iter()) {
            processed_count += 1;
            match result {
                Ok(_) => {
                    mutate_state(|state|
                        state.data.payment_processor.set_active_payment_status(
                            &payment_round.token,
                            &neuron_id,
                            PaymentStatus::Completed
                        )
                    );
                }
                Err(e) => {
                    mutate_state(|state|
                        state.data.payment_processor.set_active_payment_status(
                            &payment_round.token,
                            &neuron_id,
                            PaymentStatus::Failed(e.clone())
                        )
                    );
                }
            }
        }
        debug!(
            "ROUND ID : {} & TOKEN :{:?} - processed count {} out of {} ",
            payment_round.id,
            payment_round.token,
            processed_count,
            total_to_process
        );
    }
    info!("ROUND ID : {} & TOKEN :{:?} - FINISHED PAYMENTS", payment_round.id, payment_round.token);
}

// Create and return a future that always returns an Err
#[allow(dead_code)]
fn always_fail_future() -> impl Future<Output = Result<(), String>> {
    err("simulated failure".to_string())
}

#[cfg(test)]
mod tests {
    use std::collections::{ BTreeMap, HashMap };

    use candid::{ Nat, Principal };
    use canister_time::timestamp_millis;
    use sns_governance_canister::types::NeuronId;
    use types::{ NeuronInfo, TokenSymbol };

    use crate::{
        model::payment_processor::{ PaymentRound, PaymentStatus },
        state::{ init_state, mutate_state, read_state, RuntimeState },
    };

    use super::{ log_payment_round_metrics, update_neuron_rewards };

    fn init_runtime_state() {
        init_state(RuntimeState::default());
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
        payments.insert(neuron_id_1, (
            Nat::from(1u64),
            PaymentStatus::Failed("simulated fail".to_string()),
            1,
        ));
        payments.insert(neuron_id_2, (Nat::from(1u64), PaymentStatus::Completed, 1));
        payments.insert(neuron_id_3, (Nat::from(1u64), PaymentStatus::Completed, 1));
        payments.insert(neuron_id_4, (Nat::from(1u64), PaymentStatus::Completed, 1));
        payments.insert(neuron_id_5, (Nat::from(1u64), PaymentStatus::Completed, 1));

        let ledger_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
        let icp_symbol = TokenSymbol::parse("ICP").unwrap();

        let round = PaymentRound {
            id: 1u16,
            round_funds_total: Nat::from(100_000u64),
            tokens_to_distribute: Nat::from(94_000u64), // 5 payments + reward pool -> round pool
            fees: Nat::from(50_000u64), // 5 payments
            ledger_id,
            token: icp_symbol,
            date_initialized: timestamp_millis(),
            total_neuron_maturity: 5u64,
            payments,
            retries: 0,
        };

        let result = log_payment_round_metrics(&round);

        assert_eq!(
            result,
            "PAYMENT ROUND METRICS || round id : 1, round status : CompletedPartial, token : TokenSymbol(\"ICP\"), total : 5, successful : 4, maturity distributed : 4, round maturity : 5, retries : 0, tokens_to_distribute : 94_000"
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
        let icp_symbol = TokenSymbol::parse("ICP").unwrap();
        neuron_1_rewarded.insert(icp_symbol.clone(), 0);

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
        payments.insert(neuron_id_1.clone(), (
            Nat::from(1u64),
            PaymentStatus::Completed,
            expected_result,
        ));

        let ledger_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

        let round = PaymentRound {
            id: 1u16,
            round_funds_total: Nat::from(100_000u64),
            tokens_to_distribute: Nat::from(98_000u64), // 1 payment + 1 reward pool -> round pool transfer
            fees: Nat::from(10_000u64),
            ledger_id,
            token: icp_symbol.clone(),
            date_initialized: timestamp_millis(),
            total_neuron_maturity: 5u64,
            payments,
            retries: 0,
        };

        update_neuron_rewards(&round);

        // test 1
        read_state(|state| {
            let neuron = state.data.neuron_maturity.get(&neuron_id_1).unwrap();

            let rewarded_amount = neuron.rewarded_maturity.get(&icp_symbol).unwrap();
            assert_eq!(rewarded_amount.clone(), expected_result);
        });

        // don't strictly need to do this
        mutate_state(|state| {
            let neuron_maturity = state.data.neuron_maturity.get_mut(&neuron_id_1).unwrap();
            neuron_maturity.accumulated_maturity += 150; // 450 in total now
        });
        // use same payment round from before
        update_neuron_rewards(&round);
        let expected_result = 300u64; // two payments of 150

        read_state(|state| {
            let neuron = state.data.neuron_maturity.get(&neuron_id_1).unwrap();
            let rewarded_amount = neuron.rewarded_maturity.get(&icp_symbol).unwrap();
            assert_eq!(rewarded_amount.clone(), expected_result);
        });

        // update the neuron maturity
    }
}
