use super::init_canister;
use crate::state::replace_icrc3;
use crate::state::start_default_archive_job;
use crate::{memory::get_upgrades_memory, state::RuntimeState};
use bity_ic_canister_logger::LogEntry;
use bity_ic_canister_tracing_macros::trace;
use bity_ic_icrc3::icrc3::ICRC3;
use bity_ic_stable_memory::get_reader;
pub use gldt_swap_api_canister::Args;
use ic_cdk_macros::post_upgrade;
use tracing::info;

// use crate::migrations::types::state::RuntimeStateV0;
// use crate::model::swap_configs::SwapConfigs;
// use candid::Nat;
// use candid::Principal;
// use gldt_swap_common::swap_canister_config::FractionalizationConfig;
// use gldt_swap_common::swap_canister_config::GeneralFractionalizationConfig;
// use gldt_swap_common::swap_canister_config::SwapCanisterConfig;

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

            // // NOTE: Uncomment to change the configs
            // state.data.swap_configs = SwapConfigs {
            //     configs: vec![
            //         SwapCanisterConfig {
            //             icrc7_canister_id: Principal::from_text("g6yny-dyaaa-aaaab-qb2kq-cai").unwrap(), //1g
            //             fractionalization_config: FractionalizationConfig::General(
            //                 GeneralFractionalizationConfig {
            //                     division: 10_000_000_000,
            //                     swap_fee: Nat::from(90_000_000_u64),
            //                     ledger_id: Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").unwrap(),
            //                 },
            //             ),
            //         },

            //         SwapCanisterConfig {
            //             icrc7_canister_id: Principal::from_text("qwf5l-cqaaa-aaaad-aapma-cai").unwrap(), //10g
            //             fractionalization_config: FractionalizationConfig::General(
            //                 GeneralFractionalizationConfig {
            //                     division: 100_000_000_000,
            //                     swap_fee: Nat::from(90_000_000_u64),
            //                     ledger_id: Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").unwrap(),
            //                 },
            //             ),
            //         },
            //     ],
            // };
            // uncomment these lines if you want to do an upgrade with migration
            // let (runtime_state_v0, logs, traces, icrc3): (RuntimeStateV0, Vec<LogEntry>, Vec<LogEntry>, ICRC3) = bity_ic_serializer
            //     ::deserialize(reader)
            //     .unwrap();
            // let mut state = RuntimeState::from(runtime_state_v0);

            state.env.set_version(upgrade_args.version);
            state.env.set_commit_hash(upgrade_args.commit_hash);

            // state.data.swap_system.finalize_all_swaps();

            bity_ic_canister_logger::init_with_logs(state.env.is_test_mode(), logs, traces);
            init_canister(state);
            replace_icrc3(icrc3);
            start_default_archive_job();

            info!(version = %upgrade_args.version, "Post-upgrade complete");
        }
    }
}
