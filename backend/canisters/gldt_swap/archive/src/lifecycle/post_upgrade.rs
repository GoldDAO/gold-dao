use crate::{memory::get_upgrades_memory, state::RuntimeState};
use canister_logger::LogEntry;
use canister_tracing_macros::trace;
pub use gldt_swap_api_archive::lifecycle::Args;
use ic_cdk_macros::post_upgrade;
use stable_memory::get_reader;
use tracing::info;

use super::init_canister;

#[post_upgrade]
#[trace]
fn post_upgrade(args: Args) {
    match args {
        Args::Init(_) =>
            panic!(
                "Cannot upgrade the canister with an Init argument. Please provide an Upgrade argument."
            ),
        Args::Upgrade(upgrade_args) => {
            let memory = get_upgrades_memory();
            let reader = get_reader(&memory);

            let (mut state, logs, traces): (RuntimeState, Vec<LogEntry>, Vec<LogEntry>) = serializer
                ::deserialize(reader)
                .unwrap();

            state.env.set_version(upgrade_args.version);
            state.env.set_commit_hash(upgrade_args.commit_hash);

            canister_logger::init_with_logs(state.env.is_test_mode(), logs, traces);
            init_canister(state);

            info!(version = %upgrade_args.version, "Post-upgrade complete");
        }
    }
}
