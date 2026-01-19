use crate::lifecycle::init_canister;
use crate::memory::get_upgrades_memory;
use crate::migrations::types::state::RuntimeStateV0;
use crate::state::RuntimeState;
use bity_ic_canister_logger::LogEntry;
use bity_ic_canister_tracing_macros::trace;
use bity_ic_stable_memory::get_reader;
use buyback_burn_api::exchange_job_config::ExchangeJobConfig;
use buyback_burn_api::icpswap::ICPSwapConfig;
use buyback_burn_api::swap_config::ExchangeConfig;
pub use buyback_burn_api::Args;
use candid::Principal;
use ic_cdk_macros::post_upgrade;
use ic_ledger_types::Tokens;
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;
use tracing::info;
use types::TokenSymbol;

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
            // let (mut state, logs, traces): (RuntimeState, Vec<LogEntry>, Vec<LogEntry>) = bity_ic_serializer
            //     ::deserialize(reader)
            //     .unwrap();

            // NOTE: uncomment these lines if you want to do an upgrade with migration
            let (runtime_state_v0, logs, traces): (
                RuntimeStateV0,
                Vec<LogEntry>,
                Vec<LogEntry>,
            ) = bity_ic_serializer::deserialize(reader).unwrap();
            let mut state = RuntimeState::from(runtime_state_v0);

            let mut exchange_configs: Vec<ExchangeJobConfig> = Vec::new();
            exchange_configs.push(ExchangeJobConfig{
                token_to_sell: TokenSymbol::ICP,
                token_to_buy: TokenSymbol::GOLDAO,
                exchange: ExchangeConfig::ICPSwap(ICPSwapConfig {
                    swap_canister_id: Principal::from_text("k46ek-4qaaa-aaaag-qcyzq-cai").unwrap(),
                    zero_for_one: true,
                }),
                rate_per_interval: 793_650, // 30% per week = 0.78571428571% per interval = 0.0078571428571 scaled by 1_000_000_000
                job_interval_ms: Duration::from_secs(14400).as_millis() as u64, // 4 hours
                source_subaccount: Default::default(),
                min_amount: Tokens::from_e8s(10_000_000),
                max_amount: None,
                destination_account: None, // all the GOLDAO burned from the 0 subaccount
            });

            exchange_configs.push(ExchangeJobConfig{
                token_to_sell: TokenSymbol::ICP,
                token_to_buy: TokenSymbol::GLDT,
                exchange: ExchangeConfig::ICPSwap(ICPSwapConfig {
                    swap_canister_id: Principal::from_text("4omhz-yiaaa-aaaag-qnalq-cai").unwrap(),
                    zero_for_one: false,
                }),
                rate_per_interval: 3_571_428, // 0.39285714285% per interval = 0.0039285714285 scaled by 1_000_000_000
                job_interval_ms: Duration::from_secs(21600).as_millis() as u64, // 6 hours
                source_subaccount: Some([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
                min_amount: Tokens::from_e8s(10_000_000),
                max_amount: None,
                destination_account: Some(Principal::from_text("5aybl-v7aii-duvsu-ztemq-litdi-ly42r-iyf35-2k46p-ovynj-amtow-rae").expect("Failed to parse destination account").into()), // GLDT destination account
            });

            // NOTE: uncomment this line to clear existing exchange jobs before adding new ones
            // state.data.exchange_jobs.clear_exchange_jobs();

            for exchange_job_config in exchange_configs {
                let _ = state.data.exchange_jobs.add_exchange_job(exchange_job_config);
            }

            // end of migration code

            state.env.set_version(upgrade_args.version);
            state.env.set_commit_hash(upgrade_args.commit_hash);

            bity_ic_canister_logger::init_with_logs(state.env.is_test_mode(), logs, traces);
            init_canister(state);

            info!(version = %upgrade_args.version, "Post-upgrade complete");
        }
    }
}
