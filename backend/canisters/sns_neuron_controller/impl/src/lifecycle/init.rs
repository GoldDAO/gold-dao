use crate::lifecycle::init_canister;
use crate::state::{Data, RuntimeState};

use canister_tracing_macros::trace;
use ic_cdk_macros::init;
pub use sns_neuron_controller_api_canister::init::InitArgs;
use tracing::info;
use utils::consts::SNS_GOVERNANCE_CANISTER_ID_STAGING;
use utils::env::{CanisterEnv, Environment};

#[init]
#[trace]
fn init(args: InitArgs) {
    canister_logger::init(args.test_mode);

    let env = CanisterEnv::new(args.test_mode);
    let mut data = Data::new(
        args.ogy_sns_governance_canister_id,
        args.ogy_sns_ledger_canister_id,
        args.ogy_sns_rewards_canister_id,
        args.sns_rewards_canister_id,
        env.now(),
    );

    if args.test_mode {
        data.authorized_principals.push(env.caller());
        data.authorized_principals
            .push(SNS_GOVERNANCE_CANISTER_ID_STAGING);
    }

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
