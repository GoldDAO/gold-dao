use crate::lifecycle::init_canister;
use crate::state::{Data, RuntimeState};
pub use buyback_burn_api::Args;
use canister_tracing_macros::trace;
use ic_cdk_macros::init;
use tracing::info;
use utils::consts::SNS_GOVERNANCE_CANISTER_ID_STAGING;
use utils::env::{CanisterEnv, Environment};

#[init]
#[trace]
fn init(args: Args) {
    match args {
        Args::Init(init_args) => {
            canister_logger::init(init_args.test_mode);

            let env = CanisterEnv::new(
                init_args.test_mode,
                init_args.version,
                init_args.commit_hash,
            );
            let mut data = Data::new(
                init_args.authorized_principals,
                init_args.tokens,
                init_args.gldgov_token_info,
                init_args.buyback_burn_interval_in_secs,
                init_args.icp_swap_canister_id,
                init_args.burn_rate,
                init_args.min_burn_amount,
            );

            if init_args.test_mode {
                data.authorized_principals.push(env.caller());
                data.authorized_principals
                    .push(SNS_GOVERNANCE_CANISTER_ID_STAGING);
            }

            let runtime_state = RuntimeState::new(env, data);

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
