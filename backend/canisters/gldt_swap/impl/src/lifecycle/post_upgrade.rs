use canister_logger::LogEntry;
use canister_tracing_macros::trace;
use gldt_swap_api_canister::init::InitArgs;
use gldt_swap_common::swap::{ ArchiveDownReason, ArchiveStatus };
use ic_cdk_macros::post_upgrade;
use stable_memory::get_reader;
use tracing::info;

use crate::{ memory::get_upgrades_memory, state::RuntimeState };

use super::init_canister;

#[post_upgrade]
#[trace]
fn post_upgrade(args: InitArgs) {
    let memory = get_upgrades_memory();
    let reader = get_reader(&memory);

    let (mut runtime_state, logs, traces): (RuntimeState, Vec<LogEntry>, Vec<LogEntry>) = serializer
        ::deserialize(reader)
        .unwrap();

    runtime_state.data.version = args.version;
    runtime_state.data.should_upgrade_archives = true;
    runtime_state.data.is_archive_cron_running = false;
    runtime_state.data.archive_status = ArchiveStatus::Down(ArchiveDownReason::Upgrading);

    canister_logger::init_with_logs(runtime_state.env.is_test_mode(), logs, traces);
    init_canister(runtime_state);

    info!("Post upgrade complete.")
}
