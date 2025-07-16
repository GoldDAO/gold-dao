use crate::model::unallocated_rewards_pool::UnallocatedRewards;
use crate::state::{mutate_state, read_state};
use candid::Nat;
use canister_time::start_job_daily_at;
use futures::future::join_all;
use std::time::Duration;
use tracing::error;
use tracing::info;

pub fn start_job() {
    start_job_daily_at(15, spawn_rewards_job);
}

fn spawn_rewards_job() {
    ic_cdk::futures::spawn(process_rewards_impl())
}

async fn process_rewards_impl() {
    info!("PROCESS_REWARDS :: start");

    if !is_allowed_to_run() {
        schedule_retry(Duration::from_secs(60 * 5));
        return;
    }

    mutate_state(|s| {
        s.data.unallocated_rewards_pool.transition_to_transferring();
    });

    let reward_types = read_state(|s| s.data.stake_system.reward_types.clone());

    let unallocated_rewards_pool = read_state(|s| s.data.unallocated_rewards_pool.clone());

    let mut transfer_futures = Vec::new();
    let mut token_symbols = Vec::new();

    for reward_type in reward_types.clone().into_iter() {
        let token_ledger = reward_type.get_token_info().ledger_id;
        let ledger_fee = reward_type.get_token_info().fee;

        let future =
            unallocated_rewards_pool.transfer_part_of_rewards(token_ledger, Nat::from(ledger_fee));
        transfer_futures.push(future);
        token_symbols.push(reward_type);
    }

    let results = join_all(transfer_futures).await;

    for (token_symbol, transfer_result) in token_symbols.into_iter().zip(results.into_iter()) {
        match transfer_result {
            Ok(rewards_to_allocate) => {
                info!(
                    "PROCESS_REWARDS :: transfer successful for token: {}, rewards to allocate: {}",
                    token_symbol, rewards_to_allocate
                );
            }
            Err(err) => {
                error!(
                    "PROCESS_REWARDS :: transfer failed for token: {}, error: {}",
                    token_symbol, err
                );
            }
        }
    }

    mutate_state(|s| {
        s.data.unallocated_rewards_pool.transition_to_awaiting();
    });

    info!("PROCESS_REWARDS :: finished");
}

fn is_allowed_to_run() -> bool {
    let is_awaiting = read_state(|s| s.data.unallocated_rewards_pool.is_awaiting());

    if is_awaiting {
        info!("PROCESS_REWARDS :: reward claim already in progress");
        return true;
    }

    false
}

fn schedule_retry(delay: Duration) {
    ic_cdk_timers::set_timer(delay, || {
        ic_cdk::futures::spawn(process_rewards_impl());
    });
}
