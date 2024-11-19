use gldt_swap_common::swap::{ArchiveDownReason, ArchiveStatus, ServiceDownReason, ServiceStatus};

use crate::state::{mutate_state, read_state};

pub async fn check_service_status() -> ServiceStatus {
    // does active swap have balance
    let (archive_status, is_active_swaps_full) = read_state(|s| {
        (
            s.data.archive_status.clone(),
            s.data.swaps.is_active_swaps_capacity_full(),
        )
    });

    let status = match archive_status {
        ArchiveStatus::Down(down_reason) => {
            ServiceStatus::Down(ServiceDownReason::ArchiveRelated(down_reason))
        }
        ArchiveStatus::Upgrading => ServiceStatus::Down(ServiceDownReason::ArchiveRelated(
            ArchiveDownReason::Upgrading,
        )),
        ArchiveStatus::Initializing => ServiceStatus::Down(ServiceDownReason::ArchiveRelated(
            ArchiveDownReason::Upgrading,
        )),
        ArchiveStatus::Up => {
            if is_active_swaps_full {
                ServiceStatus::Down(ServiceDownReason::ActiveSwapCapacityFull)
            } else {
                ServiceStatus::Up
            }
        }
    };
    mutate_state(|s| s.set_service_status(status.clone()));
    status
}
