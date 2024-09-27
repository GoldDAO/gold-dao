use candid::Nat;
use canister_logger::LogEntry;
use canister_tracing_macros::trace;
use ic_cdk_macros::post_upgrade;
use sns_rewards_api_canister::Args;
use stable_memory::get_reader;
use tracing::info;
use types::TimestampMillis;

use crate::{
    memory::get_upgrades_memory,
    migrations::types::state::RuntimeStateV0,
    state::RuntimeState,
    utils::TimeInterval,
};

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

            // uncomment these lines if you want to do a normal upgrade
            let (mut state, logs, traces): (RuntimeState, Vec<LogEntry>, Vec<LogEntry>) = serializer
                ::deserialize(reader)
                .unwrap();

            // uncomment these lines if you want to do an upgrade with migration
            // let (runtime_state_v0, logs, traces): (
            //     RuntimeStateV0,
            //     Vec<LogEntry>,
            //     Vec<LogEntry>,
            // ) = serializer::deserialize(reader).unwrap();
            // let mut state = RuntimeState::from(runtime_state_v0);

            state.env.set_version(upgrade_args.version);
            state.env.set_commit_hash(upgrade_args.commit_hash);

            // Migrations

            if state.data.reward_distribution_interval.is_none() {
                state.data.reward_distribution_interval = Some(TimeInterval::default());
            }

            if state.data.reward_distribution_in_progress.is_none() {
                state.data.reward_distribution_in_progress = Some(false);
            }

            if state.data.neuron_sync_interval.is_none() {
                state.data.neuron_sync_interval = Some(TimeInterval {
                    weekday: None,
                    start_hour: 9,
                    end_hour: 11,
                });
            }

            // End migrations
            canister_logger::init_with_logs(state.env.is_test_mode(), logs, traces);
            init_canister(state);

            info!(version = %upgrade_args.version, "Post-upgrade complete");
        }
    }
}
