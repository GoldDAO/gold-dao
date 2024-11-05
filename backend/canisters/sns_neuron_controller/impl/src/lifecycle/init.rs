use crate::lifecycle::init_canister;
use crate::state::{Data, RuntimeState};

use canister_tracing_macros::trace;
use ic_cdk_macros::init;
pub use sns_neuron_controller_api_canister::Args;
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
                init_args.ogy_sns_governance_canister_id,
                init_args.ogy_sns_ledger_canister_id,
                init_args.ogy_sns_rewards_canister_id,
                init_args.sns_rewards_canister_id,
                env.now(),
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
