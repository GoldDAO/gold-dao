use crate::state::{mutate_state, read_state};
use candid::Nat;
use canister_time::run_interval;
use futures::future::join_all;
use gldt_stake_common::archive::MANAGE_ARCHIVE_CYCLE_INTERVAL;
use std::time::Duration;
use tracing::info;
use types::Cycles;
use utils::{
    canister::{deposit_cycles, get_cycles_balance},
    env::Environment,
};

pub fn start_job() {
    run_interval(
        Duration::from_millis(MANAGE_ARCHIVE_CYCLE_INTERVAL),
        spawn_transfer_job,
    );
}

pub fn spawn_transfer_job() {
    ic_cdk::spawn(handle_archive_canister_cycles())
}

async fn handle_archive_canister_cycles() {
    let is_test_mode = read_state(|s| s.env.is_test_mode());
    let archive_canisters = read_state(|s| s.data.archive_system.get_archive_canisters());
    let this_canister_cycle_balance: Cycles = read_state(|state| state.env.cycles_balance());

    let archive_canister_threshold: Cycles = if is_test_mode {
        5_000_000_000_000
    } else {
        10_000_000_000_000
    };
    let archive_canister_topup_amount: Cycles = if is_test_mode {
        1_000_000_000_000
    } else {
        10_000_000_000_000
    };

    let required_base_cycles: Cycles =
        archive_canister_threshold * ((archive_canisters.len() as u64) + 1); // all archive canisters plus this canister

    mutate_state(|s| {
        s.data.archive_system.required_cycle_balance = Nat::from(required_base_cycles);
    });
    // we dont have enough in this canister to reliably transfer to all archive canisters and preserve some cycles for the main canister
    if this_canister_cycle_balance < required_base_cycles {
        info!(
            "CYCLE MANAGER :: WARNING :: Not enough total cycles to top up all potential archive canisters. required minimum : {required_base_cycles}. current cycle balance : {this_canister_cycle_balance}"
        );
        return;
    }

    let tasks: Vec<_> = archive_canisters
        .into_iter()
        .map(|archive| {
            let archive_canister_id = archive.canister_id;
            async move {
                match get_cycles_balance(archive_canister_id).await {
                    Ok(archive_cycle_balance) => {
                        if archive_cycle_balance < archive_canister_threshold {
                            match
                                deposit_cycles(
                                    archive_canister_id,
                                    archive_canister_topup_amount
                                ).await
                            {
                                Ok(_) => {
                                    info!(
                                        "CYCLE MANAGER :: deposited cycles for archive {archive_canister_id:?}"
                                    );
                                }
                                Err(e) => {
                                    info!(
                                        "CYCLE MANAGER :: ERROR :: Failed to top up archive canister : {archive_canister_id}. with error : {e:?}"
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        info!(
                            "CYCLE MANAGER :: ERROR :: Failed to get balance of archive canister : {archive_canister_id}. with error : {e}"
                        );
                    }
                }
            }
        })
        .collect();

    // Await all tasks
    join_all(tasks).await;
}
