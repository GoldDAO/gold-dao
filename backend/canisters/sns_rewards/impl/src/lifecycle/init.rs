use candid::{ CandidType, Principal };
use ic_cdk_macros::init;
use serde::Deserialize;

use crate::state::{ init_state, RuntimeState };

#[derive(Deserialize, CandidType)]
pub struct Args {
    sns_governance_canister: Principal,
}

#[init]
fn init(args: Args) {
    let runtime_state = RuntimeState::new(args.sns_governance_canister);

    init_state(runtime_state)
}
