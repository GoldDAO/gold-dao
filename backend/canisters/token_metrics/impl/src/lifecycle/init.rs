use ic_cdk_macros::init;
pub use token_metrics_api::init::InitArgs;
use tracing::info;
use utils::env::CanisterEnv;

use crate::state::{ Data, RuntimeState };

use super::init_canister;

#[init]
fn init(args: InitArgs) {
    canister_logger::init(args.test_mode);

    let env = CanisterEnv::new(args.test_mode);
    let data = Data::new(
        args.ogy_new_ledger_canister_id,
        args.sns_governance_canister_id,
        args.super_stats_canister_id,
        args.sns_rewards_canister_id,
        args.treasury_account,
        args.foundation_accounts
    );

    let runtime_state = RuntimeState::new(env.clone(), data);

    init_canister(runtime_state);

    info!("Init complete.")
}
