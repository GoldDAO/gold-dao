use candid::Principal;
use ic_cdk_macros::init;
pub use token_metrics_api::Args;
use tracing::info;
use utils::{
    consts::{
        GOLD_1000G_CANISTER_ID,
        GOLD_100G_CANISTER_ID,
        GOLD_10G_CANISTER_ID,
        GOLD_1G_CANISTER_ID,
        STAGING_GOLD_10G_CANISTER_ID,
        STAGING_GOLD_1G_CANISTER_ID,
    },
    env::CanisterEnv,
};

use crate::state::{ Data, RuntimeState };

use super::init_canister;

#[init]
fn init(args: Args) {
    match args {
        Args::Init(init_args) => {
            canister_logger::init(init_args.test_mode);

            let gold_nft_canister: Vec<(Principal, u128)> = if init_args.test_mode {
                vec![(STAGING_GOLD_1G_CANISTER_ID, 1), (STAGING_GOLD_10G_CANISTER_ID, 10)]
            } else {
                vec![
                    (GOLD_1G_CANISTER_ID, 1),
                    (GOLD_10G_CANISTER_ID, 10),
                    (GOLD_100G_CANISTER_ID, 100),
                    (GOLD_1000G_CANISTER_ID, 1000)
                ]
            };

            let env = CanisterEnv::new(
                init_args.test_mode,
                init_args.wasm_version,
                init_args.commit_hash
            );
            let data = Data::new(
                gold_nft_canister,
                init_args.ogy_new_ledger_canister_id,
                init_args.sns_governance_canister_id,
                init_args.super_stats_canister_id,
                init_args.sns_rewards_canister_id,
                init_args.treasury_account,
                init_args.foundation_accounts
            );

            let runtime_state = RuntimeState::new(env.clone(), data);

            init_canister(runtime_state);

            info!("Init complete.")
        }
        Args::Upgrade(_) => {
            panic!(
                "Cannot initialize the canister with an Upgrade argument. Please provide an Init argument."
            );
        }
    }
}
