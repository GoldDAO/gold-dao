use candid::CandidType;
use ic_cdk_macros::init;
use serde::Deserialize;
use tracing::info;
use utils::env::CanisterEnv;

use crate::state::{ Data, RuntimeState };

use super::init_canister;

#[derive(Deserialize, CandidType)]
pub struct Args {
    test_mode: bool,
}

#[init]
fn init(args: Args) {
    canister_logger::init(args.test_mode);

    let env = CanisterEnv::new(args.test_mode);
    let data = Data::default();

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
