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
use crate::jobs::distribute_rewards::MAX_RETRIES;
use crate::state::{mutate_state, read_state};
use crate::utils::next_14_feb_seconds;
use crate::utils::next_first_wednesday_of_month_seconds;
use bity_ic_canister_time::timestamp_seconds;
use futures::FutureExt;
use std::time::Duration;
use tracing::{error, info};
use types::ledger_id;
use types::token_info;
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
    // 1. Check if neurons are syncing
    if read_state(|s| s.get_is_synchronizing_neurons()) {
        schedule_retry(Duration::from_secs(300));
        return;
    }

    // 2. Global distribution lock
    if read_state(|s| s.data.reward_distribution_in_progress.unwrap_or(false)) {
        schedule_retry(Duration::from_secs(300));
        return;
    }

    // 3. GLDT-specific lock
    if read_state(|s| s.data.gldt_distribution_in_progress).unwrap_or(false) {
        return;
    }

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
    let pending_payment_rounds =
        read_state(|state| state.data.payment_processor.get_active_rounds());

    if pending_payment_rounds.is_empty() && retry_attempt == 0 {
        create_new_gldt_payment_round().await;
    }

    let active_rounds = read_state(|state| state.data.payment_processor.get_active_rounds());

    if active_rounds.is_empty() {
        return;
    }

    for payment_round in active_rounds {
        process_payment_round(payment_round.clone(), retry_attempt).await;
    }

    let processed_payment_rounds =
        read_state(|state| state.data.payment_processor.get_active_rounds());

    if should_retry_distribution(&processed_payment_rounds) && retry_attempt < MAX_RETRIES {
        ic_cdk::futures::spawn(distribute_rewards(retry_attempt + 1));
    } else {
        finalize_distribution(processed_payment_rounds);
    }
}

use crate::jobs::distribute_rewards::transfer_funds_to_payment_round_account;
use sns_rewards_api_canister::payment_round::PaymentRound;
pub async fn create_new_gldt_payment_round() {
    let gldt_token = TokenSymbol::GLDT;
    let mut gldt_token_info = token_info!(GLDT);
    let gldt_ledger_id = ledger_id!(GLDT);
    gldt_token_info.ledger_id = gldt_ledger_id;

    let new_round_key = read_state(|state| state.data.payment_processor.next_key());

    let reward_pool_balance = fetch_reward_pool_balance(gldt_ledger_id).await;

    if reward_pool_balance == 0u64 {
        info!("[GLDT_REWARDS] No rewards available. Round will not be created with 0 balance.");
        return;
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
        Ok(valid_round) => match transfer_funds_to_payment_round_account(&valid_round).await {
            Ok(()) => {
                mutate_state(|state| {
                    state
                        .data
                        .payment_processor
                        .add_active_payment_round(valid_round);
                });
            }
            Err(e) => {
                error!("[GLDT_REWARDS] Transfer to payment round failed: {}", e);
            }
        },
        Err(s) => {
            error!(
                "[GLDT_REWARDS] Invalid payment round {}: {}",
                new_round_key, s
            );
        }
    }
}
