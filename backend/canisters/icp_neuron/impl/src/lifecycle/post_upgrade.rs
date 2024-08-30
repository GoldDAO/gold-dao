use crate::lifecycle::init_canister;
use crate::memory::get_upgrades_memory;
use crate::state::RuntimeState;
use crate::Args;
use candid::CandidType;
use canister_logger::LogEntry;
use canister_tracing_macros::trace;
use ic_cdk_macros::post_upgrade;
use serde::{Deserialize, Serialize};
use stable_memory::get_reader;
use tracing::info;
use types::BuildVersion;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UpgradeArgs {
    pub wasm_version: BuildVersion,
    pub commit_hash: String,
}

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

            state.env.set_version(upgrade_args.wasm_version);
            state.env.set_commit_hash(upgrade_args.commit_hash);

            // uncomment these lines if you want to do an upgrade with migration
            // let (runtime_state_v0, logs, traces): (
            //     RuntimeStateV0,
            //     Vec<LogEntry>,
            //     Vec<LogEntry>,
            // ) = serializer::deserialize(reader).unwrap();
            // let runtime_state = RuntimeState::from(runtime_state_v0);

            canister_logger::init_with_logs(state.env.is_test_mode(), logs, traces);
            init_canister(state);

            info!(version = %upgrade_args.wasm_version, "Post-upgrade complete");
        }
    }
}
