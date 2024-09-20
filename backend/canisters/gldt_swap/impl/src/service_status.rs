use candid::Nat;
use gldt_swap_common::swap::{ ArchiveDownReason, ArchiveStatus, ServiceDownReason, ServiceStatus };
use icrc_ledger_canister_c2c_client::icrc1_balance_of;
use icrc_ledger_types::icrc1::account::Account;
use tracing::debug;
use utils::{ env::Environment, retry_async::retry_async };

use crate::state::{ mutate_state, read_state };

pub async fn check_service_status() -> ServiceStatus {
    // the canister may fail to create its initial archive or update so we should return early if that is the case
    let this_canister_id = read_state(|s| s.env.canister_id());
    let required_ogy_threshold = read_state(|s| s.get_required_ogy_for_canister());
    let ogy_ledger_id = read_state(|s| s.data.ogy_ledger_id);
    let balance = match
        retry_async(
            ||
                icrc1_balance_of(ogy_ledger_id, Account {
                    owner: this_canister_id,
                    subaccount: None,
                }),
            3
        ).await
    {
        Ok(balance) => {
            mutate_state(|s| {
                s.data.ogy_balance = balance.clone();
            });
            balance
        }
        Err(_) => {
            debug!("ERROR: Can't get OGY balance after 3 retries. check OGY ledger is online");
            Nat::from(0u64)
        }
    };
    // is ogy balance more than threshold
    // does active swap have balance
    let (archive_status, is_active_swaps_full) = read_state(|s| (
        s.data.archive_status.clone(),
        s.data.swaps.is_active_swaps_capacity_full(),
    ));

    let status = match archive_status {
        ArchiveStatus::Down(down_reason) => {
            ServiceStatus::Down(ServiceDownReason::ArchiveRelated(down_reason))
        }
        ArchiveStatus::Upgrading =>
            ServiceStatus::Down(ServiceDownReason::ArchiveRelated(ArchiveDownReason::Upgrading)),
        ArchiveStatus::Initializing =>
            ServiceStatus::Down(ServiceDownReason::ArchiveRelated(ArchiveDownReason::Upgrading)),
        ArchiveStatus::Up => {
            if balance < required_ogy_threshold {
                ServiceStatus::Down(
                    ServiceDownReason::LowOrigynToken(
                        format!(
                            "The canister OGY balance of {balance} does not meet the threshold of {required_ogy_threshold}"
                        )
                    )
                )
            } else if is_active_swaps_full {
                ServiceStatus::Down(ServiceDownReason::ActiveSwapCapacityFull)
            } else {
                ServiceStatus::Up
            }
        }
    };
    mutate_state(|s| s.set_service_status(status.clone()));
    status
}
