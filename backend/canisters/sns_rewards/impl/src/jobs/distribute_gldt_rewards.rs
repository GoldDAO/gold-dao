/*!
# SNS reward distribution

- fn distribute_rewards
Distributes reward tokens based on a neuron's total staked power.
GLDT rewards are distributed on the FIRST WEDNESDAY of every month.

- Sub accounts
reward pool - [0u8;32] -> holds reward tokens (GLDT) before distribution.
neuron / user reward - [u8;32] -> target subaccount for rewards, derived from the NeuronId.

- Concurrency & Locks
Global and token-specific locks ensure that distribution jobs do not overlap with
neuron synchronization or other active reward cycles.
*/

use crate::jobs::distribute_rewards::distribute_rewards;
use crate::jobs::distribute_rewards::fetch_reward_pool_balance;
use crate::jobs::distribute_rewards::finalize_distribution;
use crate::jobs::distribute_rewards::process_payment_round;
use crate::jobs::distribute_rewards::should_retry_distribution;
use crate::jobs::distribute_rewards::transfer_funds_to_payment_round_account;
use crate::jobs::distribute_rewards::MAX_RETRIES;
use crate::state::{mutate_state, read_state};
use crate::utils::next_14_feb_seconds;
use crate::utils::next_first_wednesday_of_month_seconds;
use bity_ic_canister_time::timestamp_seconds;
use sns_rewards_api_canister::payment_round::PaymentRound;
use std::time::Duration;
use tracing::{error, info};
use types::TokenSymbol;

pub fn start_job() {
    let now = timestamp_seconds();
    let next_first_wednesday_of_month_opt = next_first_wednesday_of_month_seconds(now, 12);

    info!(
        "Scheduling next GLDT reward distribution job. Now: {:?}, Next First Wednesday of Month: {:?}",
        now,
        next_first_wednesday_of_month_opt
    );

    match next_first_wednesday_of_month_opt {
        Ok(next_first_wednesday_of_month) => {
            let duration = next_first_wednesday_of_month.saturating_sub(now);
            if duration == 0 {
                error!("Calculated duration for GLDT reward distribution is 0. Not scheduling.");
                return;
            }
            ic_cdk_timers::set_timer(Duration::from_secs(duration), run);
            info!(
                "Scheduled GLDT reward distribution job in {:?} seconds",
                duration
            );
        }
        Err(e) => {
            error!("Failed to schedule GLDT reward distribution job: {:?}", e);
        }
    }
}

pub fn run() {
    ic_cdk::futures::spawn(run_async());
}

async fn run_async() {
    run_distribution().await;
    start_job();
}

pub fn run_once() {
    ic_cdk::futures::spawn(run_distribution());
}

pub fn start_job_14_feb() {
    let now = timestamp_seconds();

    let next_14_feb_opt = next_14_feb_seconds(12);

    info!(
        "Scheduling next GLDT reward distribution job. Now: {:?}, 14 Feb: {:?}",
        now, next_14_feb_opt
    );

    match next_14_feb_opt {
        Ok(next_14_feb) => {
            let duration = next_14_feb.saturating_sub(now);

            if duration == 0 {
                error!("Calculated duration for GLDT reward distribution is 0. Not scheduling.");
                return;
            }

            ic_cdk_timers::set_timer(Duration::from_secs(duration), run_once);

            info!(
                "Scheduled GLDT reward distribution job in {:?} seconds",
                duration
            );
        }
        Err(e) => {
            error!("Failed to schedule GLDT reward distribution job: {:?}", e);
        }
    }
}

pub async fn run_distribution() {
    ic_cdk::println!("[GLDT_REWARDS] run_distribution started");
    // 1. Check if neurons are syncing
    if read_state(|s| s.get_is_synchronizing_neurons()) {
        ic_cdk::println!("[GLDT_REWARDS] Neurons are syncing, retrying in 300s");
        schedule_retry(Duration::from_secs(300));
        return;
    }

    // 2. Global distribution lock
    if read_state(|s| s.data.reward_distribution_in_progress.unwrap_or(false)) {
        ic_cdk::println!("[GLDT_REWARDS] Global distribution lock active, retrying in 300s");
        schedule_retry(Duration::from_secs(300));
        return;
    }

    // 3. GLDT-specific lock
    if read_state(|s| s.data.gldt_distribution_in_progress).unwrap_or(false) {
        ic_cdk::println!("[GLDT_REWARDS] GLDT distribution already in progress, skipping");
        return;
    }

    ic_cdk::println!("[GLDT_REWARDS] Acquiring GLDT distribution lock");
    mutate_state(|s| {
        s.data.gldt_distribution_in_progress = Some(true);
    });

    distribute_gldt_rewards(0).await;

    ic_cdk::println!("[GLDT_REWARDS] Releasing GLDT distribution lock");

    mutate_state(|s| {
        s.data.gldt_distribution_in_progress = Some(false);
    });
}

fn schedule_retry(delay: Duration) {
    ic_cdk_timers::set_timer(delay, move || {
        run_once();
    });
}

pub async fn distribute_gldt_rewards(retry_attempt: u8) {
    ic_cdk::println!("[GLDT_REWARDS] distribute_gldt_rewards called, retry_attempt: {}", retry_attempt);
    let pending_payment_rounds =
        read_state(|state| state.data.payment_processor.get_active_rounds());

    ic_cdk::println!("[GLDT_REWARDS] Pending payment rounds: {}", pending_payment_rounds.len());
    if pending_payment_rounds.is_empty() && retry_attempt == 0 {
        ic_cdk::println!("[GLDT_REWARDS] No pending rounds, creating new GLDT payment round");
        if create_new_gldt_payment_round().await.is_err() {
            ic_cdk::println!("[GLDT_REWARDS] ERROR: Failed to create new GLDT payment round");
            error!("[GLDT_REWARDS] Failed to create new GLDT payment round");
            return;
        }
    }

    let active_rounds = read_state(|state| state.data.payment_processor.get_active_rounds());
    ic_cdk::println!("[GLDT_REWARDS] Active rounds after creation attempt: {}", active_rounds.len());

    if active_rounds.is_empty() {
        ic_cdk::println!("[GLDT_REWARDS] No active rounds to process, exiting");
        return;
    }

    for payment_round in active_rounds {
        ic_cdk::println!("[GLDT_REWARDS] Processing payment round: {:?}", payment_round.id);
        process_payment_round(payment_round.clone(), retry_attempt).await;
    }

    let processed_payment_rounds =
        read_state(|state| state.data.payment_processor.get_active_rounds());

    if should_retry_distribution(&processed_payment_rounds) && retry_attempt < MAX_RETRIES {
        ic_cdk::println!("[GLDT_REWARDS] Retrying distribution, attempt: {}", retry_attempt + 1);
        ic_cdk::futures::spawn(distribute_rewards(retry_attempt + 1));
    } else {
        ic_cdk::println!("[GLDT_REWARDS] Finalizing distribution");
        finalize_distribution(processed_payment_rounds);
    }
}

pub async fn create_new_gldt_payment_round() -> Result<(), String> {
    ic_cdk::println!("[GLDT_REWARDS] create_new_gldt_payment_round started");
    let gldt_token = TokenSymbol::GLDT;
    let gldt_token_info = read_state(|s| s.data.tokens.get(&gldt_token).copied())
        .ok_or_else(|| format!("Token info for type {gldt_token:?} not found in state"))?;
    let gldt_ledger_id = gldt_token_info.ledger_id;
    ic_cdk::println!("[GLDT_REWARDS] GLDT ledger ID: {:?}", gldt_ledger_id);

    let new_round_key = read_state(|state| state.data.payment_processor.next_key());
    ic_cdk::println!("[GLDT_REWARDS] New round key: {:?}", new_round_key);

    let reward_pool_balance = fetch_reward_pool_balance(gldt_ledger_id).await;
    ic_cdk::println!("[GLDT_REWARDS] Reward pool balance: {}", reward_pool_balance);

    if reward_pool_balance == 0u64 {
        ic_cdk::println!("[GLDT_REWARDS] Reward pool is empty, skipping round creation");
        info!("[GLDT_REWARDS] No rewards available. Round will not be created with 0 balance.");
        return Ok(());
    }

    let neuron_data = read_state(|state| state.data.neuron_system.neuron_maturity.clone());

    let new_round = PaymentRound::new(
        new_round_key,
        reward_pool_balance,
        gldt_token_info,
        gldt_token,
        neuron_data,
    );

    match new_round {
        Ok(valid_round) => {
            ic_cdk::println!("[GLDT_REWARDS] Payment round created, transferring funds to round account");
            match transfer_funds_to_payment_round_account(&valid_round).await {
                Ok(()) => {
                    ic_cdk::println!("[GLDT_REWARDS] Funds transferred successfully, adding round to active rounds");
                    mutate_state(|state| {
                        state
                            .data
                            .payment_processor
                            .add_active_payment_round(valid_round);
                    });
                }
                Err(e) => {
                    ic_cdk::println!("[GLDT_REWARDS] ERROR: Transfer to payment round failed: {}", e);
                    error!("[GLDT_REWARDS] Transfer to payment round failed: {}", e);
                }
            }
        }
        Err(s) => {
            ic_cdk::println!("[GLDT_REWARDS] ERROR: Invalid payment round {}: {}", new_round_key, s);
            error!(
                "[GLDT_REWARDS] Invalid payment round {}: {}",
                new_round_key, s
            );
        }
    };

    Ok(())
}
