use crate::lifecycle::init_canister;
use crate::state::{ Data, RuntimeState };

pub use buyback_burn_canister::init::InitArgs;
use canister_tracing_macros::trace;
use ic_cdk_macros::init;
use tracing::info;
use utils::consts::SNS_GOVERNANCE_CANISTER_ID_STAGING;
use utils::env::{ CanisterEnv, Environment };

#[init]
#[trace]
fn init(args: InitArgs) {
    canister_logger::init(args.test_mode);

    let env = CanisterEnv::new(args.test_mode);
    let mut data = Data::new(
        args.authorized_principals,
        args.tokens,
        args.gldgov_ledger_canister_id,
        args.sns_governance_canister_id,
        args.burn_rate,
        args.min_icp_burn_amount,
        args.burn_interval_in_secs,
        env.canister_id()
    );

    if args.test_mode {
        data.authorized_principals.push(env.caller());
        data.authorized_principals.push(SNS_GOVERNANCE_CANISTER_ID_STAGING);
    }

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
