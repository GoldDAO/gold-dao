use crate::state::{ Data, RuntimeState };
use candid::CandidType;
use canister_tracing_macros::trace;
use ic_cdk_macros::init;
use serde::Deserialize;
use tracing::info;
use utils::consts::{
    STAGING_GOLD_1G_CANISTER_ID,
    STAGING_GOLD_10G_CANISTER_ID,
    GOLD_1G_CANISTER_ID,
    GOLD_10G_CANISTER_ID,
    GOLD_100G_CANISTER_ID,
    GOLD_1000G_CANISTER_ID,
};
use utils::env::CanisterEnv;
use candid::Principal;
use crate::lifecycle::init_canister;

#[derive(Deserialize, CandidType, Debug)]
pub struct InitArgs {
    test_mode: bool,
}

#[init]
#[trace]
fn init(init_args: Option<InitArgs>) {
    ic_cdk::api::print(format!("init_args : {init_args:?}"));

    let args = init_args.ok_or("Must provide init arguments.".to_string()).unwrap();
    canister_logger::init(args.test_mode);

    let gold_nft_canister: Vec<(Principal, u128)> = if args.test_mode {
        vec![(STAGING_GOLD_1G_CANISTER_ID, 1), (STAGING_GOLD_10G_CANISTER_ID, 10)]
    } else {
        vec![
            (GOLD_1G_CANISTER_ID, 1),
            (GOLD_10G_CANISTER_ID, 10),
            (GOLD_100G_CANISTER_ID, 100),
            (GOLD_1000G_CANISTER_ID, 1000)
        ]
    };

    let env = CanisterEnv::new(args.test_mode);
    let data = Data::new(gold_nft_canister);

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
