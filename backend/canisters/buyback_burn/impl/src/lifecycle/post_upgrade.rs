use std::time::Duration;

use crate::types::GLDT_BUYING_POOL;
use bity_ic_canister_logger::LogEntry;
use bity_ic_canister_tracing_macros::trace;
use bity_ic_stable_memory::get_reader;
use buyback_burn_api::exchange_job_config::ExchangeJobConfig;
use buyback_burn_api::icpswap::ICPSwapConfig;
use buyback_burn_api::post_transfer_action::PostTransferAction;
use buyback_burn_api::swap_config::ExchangeConfig;
use buyback_burn_api::swap_constraint::SwapConstraint;
pub use buyback_burn_api::Args;
use candid::Principal;
use ic_cdk_macros::post_upgrade;
use ic_ledger_types::Tokens;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use tracing::info;
use types::TokenSymbol;
use utils::consts::OGY_GOVERNANCE_CANISTER_ID;

use crate::lifecycle::init_canister;
use crate::memory::get_upgrades_memory;
use crate::migrations::types::state::RuntimeStateV0;
use crate::state::RuntimeState;

pub const OGY_NEURON_SUBACCOUNT: Subaccount = [
    191, 148, 26, 66, 237, 229, 193, 81, 59, 135, 55, 86, 119, 227, 15, 230, 23, 74, 95, 121, 11,
    229, 133, 2, 144, 24, 46, 191, 163, 181, 247, 77,
];

#[post_upgrade]
#[trace]
fn post_upgrade(args: Args) {
    match args {
        Args::Init(_) => panic!(
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
            let (runtime_state_v0, logs, traces): (RuntimeStateV0, Vec<LogEntry>, Vec<LogEntry>) =
                bity_ic_serializer::deserialize(reader).unwrap();

            let mut state = RuntimeState::from(runtime_state_v0);
            let min_swap_amount = Tokens::from_e8s(10_000_000);

            // Job id - 2 (highest priority): ICP->GOLDAO, only swap if >= 500 GOLDAO per ICP
            let goldao_exchange_job_config = ExchangeJobConfig {
                token_to_sell: TokenSymbol::ICP,
                token_to_buy: TokenSymbol::GOLDAO,
                exchange: ExchangeConfig::ICPSwap(ICPSwapConfig {
                    swap_canister_id: Principal::from_text("k46ek-4qaaa-aaaag-qcyzq-cai").unwrap(),
                    zero_for_one: true,
                }),
                rate_per_interval: 2_380_950, // 1/42 of the week
                job_interval_ms: Duration::from_secs(14400).as_millis() as u64,
                source_subaccount: None,
                min_amount: min_swap_amount,
                max_amount: None,
                destination_account: None,
                constraints: vec![SwapConstraint::MinBuyRatio(500)],
                post_transfer_action: None,
            };
            let _ = state
                .data
                .exchange_jobs
                .add_exchange_job_no_timer(goldao_exchange_job_config);

            // Job id 3: ICP->OGY, only swap if >= 1000 OGY per ICP (skipped if job above runs)
            let ogy_exchange_job_config = ExchangeJobConfig {
                token_to_sell: TokenSymbol::ICP,
                token_to_buy: TokenSymbol::OGY,
                exchange: ExchangeConfig::ICPSwap(ICPSwapConfig {
                    swap_canister_id: Principal::from_text("ttnzy-lyaaa-aaaag-qj2bq-cai").unwrap(),
                    zero_for_one: false,
                }),
                rate_per_interval: 2_380_950, // 1/42 of the week
                job_interval_ms: Duration::from_secs(14400).as_millis() as u64,
                source_subaccount: None,
                min_amount: min_swap_amount,
                max_amount: None,
                constraints: vec![SwapConstraint::MinBuyRatio(1000)],
                destination_account: Some(Account {
                    owner: OGY_GOVERNANCE_CANISTER_ID,
                    subaccount: Some(OGY_NEURON_SUBACCOUNT),
                }),
                post_transfer_action: Some(PostTransferAction::SnsClaimOrRefresh {
                    governance_canister_id: OGY_GOVERNANCE_CANISTER_ID,
                    neuron_subaccount: OGY_NEURON_SUBACCOUNT,
                }),
            };
            let _ = state
                .data
                .exchange_jobs
                .add_exchange_job_no_timer(ogy_exchange_job_config);

            // update the rate - divide by 2 current rate since we have two jobs now running
            state
                .data
                .exchange_jobs
                .exchange_jobs
                .get_mut(&1)
                .expect("GLDT exchange job must exist")
                .source_subaccount = Some(GLDT_BUYING_POOL);

            // NOTE: uncomment this line to clear existing exchange jobs before adding new ones
            // state.data.exchange_jobs.clear_exchange_jobs();

            // for exchange_job_config in exchange_configs {
            //     let _ = state.data.exchange_jobs.add_exchange_job(exchange_job_config);
            // }
            // end of migration code

            // NOTE: remove the GOLDAO exchange job & burn all the left balance
            // state.data.exchange_jobs.exchange_jobs.remove(&1);

            state.env.set_version(upgrade_args.version);
            state.env.set_commit_hash(upgrade_args.commit_hash);

            bity_ic_canister_logger::init_with_logs(state.env.is_test_mode(), logs, traces);
            init_canister(state);

            info!(version = %upgrade_args.version, "Post-upgrade complete");
        }
    }
}
