use crate::state::replace_icrc3;
use crate::state::start_default_archive_job;
use crate::{
    memory::get_upgrades_memory, migrations::types::state::RuntimeStateV0, state::RuntimeState,
};
use bity_ic_icrc3::icrc3::ICRC3;
use candid::Principal;
use canister_logger::LogEntry;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::Args;
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

            // uncomment these lines if you want to do a normal upgrade
            let (mut state, logs, traces, icrc3): (RuntimeState, Vec<LogEntry>, Vec<LogEntry>, ICRC3) = bity_ic_serializer
                ::deserialize(reader)
                .unwrap();

            // uncomment these lines if you want to do an upgrade with migration
            // let (runtime_state_v0, logs, traces, icrc3): (RuntimeStateV0, Vec<LogEntry>, Vec<LogEntry>, ICRC3) = bity_ic_serializer
            //     ::deserialize(reader)
            //     .unwrap();
            // let mut state = RuntimeState::from(runtime_state_v0);

            if !state.env.is_test_mode(){
                state.data.authorized_principals = vec![
                    Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").expect("Principal cannot be parsed"),
                    Principal::from_text("wmy6h-hcvng-76kf7-uj5m2-tgm3w-nmo7m-avslw-qmsrg-32yuq-uccuk-5ae").expect("Principal cannot be parsed")
                    ];
            }

            state.env.set_version(upgrade_args.version);
            state.env.set_commit_hash(upgrade_args.commit_hash);

            canister_logger::init_with_logs(state.env.is_test_mode(), logs, traces);
            init_canister(state);
            replace_icrc3(icrc3);
            start_default_archive_job();

            info!(version = %upgrade_args.version, "Post-upgrade complete");
        }
    }
}
