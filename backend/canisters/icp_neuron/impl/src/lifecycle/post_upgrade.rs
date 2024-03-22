use crate::lifecycle::init_canister;
use crate::memory::get_upgrades_memory;
use crate::migrations::types::state::RuntimeStateV0;
use crate::state::RuntimeState;
use canister_logger::LogEntry;
use canister_tracing_macros::trace;
use ic_cdk_macros::post_upgrade;
use stable_memory::get_reader;
use tracing::info;

#[post_upgrade]
#[trace]
fn post_upgrade() {
    let memory = get_upgrades_memory();
    let reader = get_reader(&memory);

    // let (runtime_state, logs, traces): (RuntimeState, Vec<LogEntry>, Vec<LogEntry>) = serializer
    //     ::deserialize(reader)
    //     .unwrap();

    // uncomment these lines if you want to do an upgrade with migration
    let (runtime_state_v0, logs, traces): (
        RuntimeStateV0,
        Vec<LogEntry>,
        Vec<LogEntry>,
    ) = serializer::deserialize(reader).unwrap();
    let runtime_state = RuntimeState::from(runtime_state_v0);

    canister_logger::init_with_logs(runtime_state.env.is_test_mode(), logs, traces);
    init_canister(runtime_state);

    info!("Post upgrade complete.")
}
