use crate::lifecycle::init_canister;
use crate::memory::get_upgrades_memory;
use crate::state::RuntimeState;
pub use buyback_burn_api::Args;
use canister_logger::LogEntry;
use canister_tracing_macros::trace;
use ic_cdk_macros::post_upgrade;
use stable_memory::get_reader;
use tracing::info;
// use crate::migrations::types::state::RuntimeStateV0;

#[post_upgrade]
#[trace]
fn post_upgrade(args: Args) {
    match args {
        Args::Init(_) =>
            panic!(
                "Cannot upgrade the canister with an Init argument. Please provide an Upgrade argument."
            ),
        Args::Upgrade(upgrade_args) => {
            info!("Post-upgrade starting with args: {:?}", upgrade_args);
            let memory = get_upgrades_memory();
            let reader = get_reader(&memory);

            // NOTE: uncomment these lines if you want to do a normal upgrade
            let (mut state, logs, traces): (RuntimeState, Vec<LogEntry>, Vec<LogEntry>) = serializer
                ::deserialize(reader)
                .unwrap();

            // NOTE: uncomment these lines if you want to do an upgrade with migration
            // let (runtime_state_v0, logs, traces): (
            //     RuntimeStateV0,
            //     Vec<LogEntry>,
            //     Vec<LogEntry>,
            // ) = serializer::deserialize(reader).unwrap();
            // let mut state = RuntimeState::from(runtime_state_v0);

            state.env.set_version(upgrade_args.version);
            state.env.set_commit_hash(upgrade_args.commit_hash);

            canister_logger::init_with_logs(state.env.is_test_mode(), logs, traces);
            init_canister(state);

            info!(version = %upgrade_args.version, "Post-upgrade complete");
        }
    }
}
