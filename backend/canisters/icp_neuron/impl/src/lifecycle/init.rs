use crate::lifecycle::init_canister;
use crate::state::{ Data, RuntimeState };
use candid::CandidType;
use canister_tracing_macros::trace;
use ic_cdk_macros::init;
use serde::Deserialize;
use tracing::info;
use utils::consts::SNS_GOVERNANCE_CANISTER_ID_STAGING;
use utils::env::{ CanisterEnv, Environment };

#[derive(Deserialize, CandidType, Debug)]
pub struct InitArgs {
    test_mode: bool,
}

#[init]
#[trace]
fn init(init_args: Option<InitArgs>) {
    let args = init_args.ok_or("Must provide init arguments.".to_string()).unwrap();
    canister_logger::init(args.test_mode);

    let env = CanisterEnv::new(args.test_mode);
    let mut data = Data::new();

    if args.test_mode {
        data.authorized_principals.push(env.caller());
        data.authorized_principals.push(SNS_GOVERNANCE_CANISTER_ID_STAGING);
    }

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
