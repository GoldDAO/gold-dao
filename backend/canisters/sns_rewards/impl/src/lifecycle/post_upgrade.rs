use canister_logger::LogEntry;
use canister_tracing_macros::trace;
use ic_cdk_macros::post_upgrade;
use stable_memory::get_reader;
use tracing::info;

use crate::{ memory::get_upgrades_memory, state::RuntimeState, utils::TimeInterval };

use super::init_canister;

#[post_upgrade]
#[trace]
fn post_upgrade() {
    let memory = get_upgrades_memory();
    let reader = get_reader(&memory);

    let (mut runtime_state, logs, traces): (RuntimeState, Vec<LogEntry>, Vec<LogEntry>) = serializer
        ::deserialize(reader)
        .unwrap();

    // Migrations
    runtime_state.data.reward_distribution_interval = Some(TimeInterval::default());

    if runtime_state.data.reward_distribution_in_progress.is_some() {
        runtime_state.data.reward_distribution_in_progress =
            runtime_state.data.reward_distribution_in_progress;
    } else {
        runtime_state.data.reward_distribution_in_progress = Some(false);
    }
    runtime_state.data.neuron_sync_interval = Some(TimeInterval {
        weekday: None,
        start_hour: 9,
        end_hour: 11,
    });
    // End migrations
    canister_logger::init_with_logs(runtime_state.env.is_test_mode(), logs, traces);
    init_canister(runtime_state);

    info!("Post upgrade complete.")
}
