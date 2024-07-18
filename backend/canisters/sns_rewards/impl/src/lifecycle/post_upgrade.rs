use candid::Nat;
use canister_logger::LogEntry;
use canister_tracing_macros::trace;
use ic_cdk_macros::post_upgrade;
use stable_memory::get_reader;
use tracing::info;
use types::TimestampMillis;

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

    if runtime_state.data.daily_gldgov_burn_rate.is_none() {
        runtime_state.data.daily_gldgov_burn_rate = None;
    } else {
        runtime_state.data.daily_gldgov_burn_rate = runtime_state.data.daily_gldgov_burn_rate;
    }

    if runtime_state.data.last_daily_gldgov_burn.is_none() {
        runtime_state.data.last_daily_gldgov_burn = None;
    } else {
        runtime_state.data.last_daily_gldgov_burn = runtime_state.data.last_daily_gldgov_burn;
    }

    if runtime_state.data.reward_distribution_interval.is_none() {
        runtime_state.data.reward_distribution_interval = Some(TimeInterval::default());
    } else {
        runtime_state.data.reward_distribution_interval =
            runtime_state.data.reward_distribution_interval;
    }

    if runtime_state.data.reward_distribution_in_progress.is_none() {
        runtime_state.data.reward_distribution_in_progress = Some(false);
    } else {
        runtime_state.data.reward_distribution_in_progress =
            runtime_state.data.reward_distribution_in_progress;
    }

    if runtime_state.data.neuron_sync_interval.is_none() {
        runtime_state.data.neuron_sync_interval = Some(TimeInterval {
            weekday: None,
            start_hour: 9,
            end_hour: 11,
        });
    } else {
        runtime_state.data.neuron_sync_interval = runtime_state.data.neuron_sync_interval;
    }

    // End migrations
    canister_logger::init_with_logs(runtime_state.env.is_test_mode(), logs, traces);
    init_canister(runtime_state);

    info!("Post upgrade complete.")
}
