/*!
# Runs once when
- init 
- upgrading
- the timer is almost irrelevent since a bool flag is_archive_cron_running is never reset except for in post_upgrade.rs

*/

use crate::{
    create_archive_canister,
    state::{ mutate_state, read_state },
    update_archive_canisters,
};
use candid::Nat;
use gldt_swap_common::{ archive::ArchiveCanister, swap::{ ArchiveDownReason, ArchiveStatus } };
use ic_cdk::trap;
use tracing::{ debug, info };
use canister_time::run_once;

pub fn start_job() {
    run_once(spawn_transfer_job);
}

pub fn spawn_transfer_job() {
    let is_running = read_state(|s| s.data.is_archive_cron_running);
    if is_running {
        return;
    }
    ic_cdk::spawn(manage_archives())
}

async fn manage_archives() {
    mutate_state(|s| {
        s.data.should_upgrade_archives = true;
    });
    let num_archive_canisters = read_state(|s| s.data.swaps.get_total_archive_canisters());
    if num_archive_canisters == 0 {
        match create_archive_canister().await {
            Ok(principal) => {
                mutate_state(|s| {
                    s.data.swaps.set_new_archive_canister(ArchiveCanister {
                        canister_id: principal,
                        start_index: Nat::from(0u64),
                        end_index: None,
                    });
                    s.set_archive_status(ArchiveStatus::Up)
                });
                info!("SUCCESS:: initial archive canister created : {principal:?}");
            }
            Err(e) => {
                mutate_state(|s| {
                    s.set_archive_status(
                        ArchiveStatus::Down(
                            ArchiveDownReason::InitializingFirstArchiveFailed(e.clone())
                        )
                    );
                });

                trap(&e);
            }
        }
        return;
    }

    let should_upgrade_archives = read_state(|s| s.data.should_upgrade_archives);

    if should_upgrade_archives {
        match update_archive_canisters().await {
            Ok(_) => {
                info!("SUCCESS : archive upgrade - all archives upgraded successfully");
                mutate_state(|s| s.set_archive_status(ArchiveStatus::Up));
            }
            Err(errors) => {
                for e in errors {
                    debug!(e);
                    mutate_state(|s|
                        s.set_archive_status(
                            ArchiveStatus::Down(ArchiveDownReason::UpgradingArchivesFailed(e))
                        )
                    );
                }
            }
        }
        mutate_state(|s| {
            s.data.should_upgrade_archives = false;
        });
    }
}
