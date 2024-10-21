/*!
# tarnsfers GLDT swap fees that have accumulated in the fee sub account GLDT_FEE_ACCOUNT to x every 12 hours

*/

use crate::state::{ mutate_state, read_state };
use candid::Nat;
use canister_time::{ run_interval, run_now_then_interval, HOUR_IN_MS, MINUTE_IN_MS };
use futures::future::join_all;
use utils::{ canister::{ deposit_cycles, get_cycles_balance }, env::Environment };
use std::time::Duration;
use tracing::{ debug, info };
use types::{ Cycles, Milliseconds };

const MANAGE_ARCHIVE_CYCLE_INTERVAL: Milliseconds = MINUTE_IN_MS * 10;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(MANAGE_ARCHIVE_CYCLE_INTERVAL), spawn_transfer_job);
}

pub fn spawn_transfer_job() {
    ic_cdk::spawn(handle_archive_canister_cycles())
}

async fn handle_archive_canister_cycles() {
    let is_test_mode = read_state(|s| s.env.is_test_mode());
    let archive_canisters = read_state(|s| s.data.swaps.get_archive_canisters());
    let this_canister_cycle_balance: Cycles = read_state(|state| state.env.cycles_balance());

    let archive_canister_threshold: Cycles = if is_test_mode {
        2_000_000_000_000
    } else {
        10_000_000_000_000
    };
    let archive_canister_topup_amount: Cycles = if is_test_mode {
        1_000_000_000_000
    } else {
        10_000_000_000_000
    };

    let swap_canister_required_base: Cycles =
        archive_canister_threshold * ((archive_canisters.len() as u64) + 1); // all archive canisters plus this canister

    mutate_state(|s| {
        s.data.required_cycle_balance = Nat::from(swap_canister_required_base);
    });
    // we dont have enough in this canister to reliably transfer to all archive canisters and preserve some cycles for the main swap canister
    if this_canister_cycle_balance < swap_canister_required_base {
        debug!(
            "CYCLE MANAGER :: Not enough total cycles to top up all potential archive canisters. required minimum : {swap_canister_required_base}. current cycle balance : {this_canister_cycle_balance}"
        );
        return ();
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
                                    info!("SUCCESS : deposited cycles");
                                }
                                Err(e) => {
                                    debug!(
                                        "ERROR : Failed to top up archive canister : {archive_canister_id}. with error : {e:?}"
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        // trace(&format!("ERROR : archive {archive_canister_id} : has error {e:?}"));
                        debug!(
                            "ERROR : Failed to get balance of archive canister : {archive_canister_id}. with error : {e}"
                        );
                    }
                }
            }
        })
        .collect();

    // Await all tasks
    join_all(tasks).await;
}
