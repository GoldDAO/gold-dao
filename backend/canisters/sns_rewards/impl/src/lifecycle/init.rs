use candid::CandidType;
use ic_cdk_macros::init;
use serde::Deserialize;
use tracing::info;
use utils::{consts::{OGY_LEDGER_CANISTER_ID_STAGING, ICP_LEDGER_CANISTER_ID_STAGING, ICP_LEDGER_CANISTER_ID, OGY_LEDGER_CANISTER_ID, SNS_LEDGER_CANISTER_ID, SNS_LEDGER_CANISTER_ID_STAGING}, env::CanisterEnv};

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
    let mut data = Data::default();

    // use staging canister ids
    data.icp_ledger_canister_id = if args.test_mode { ICP_LEDGER_CANISTER_ID_STAGING } else { ICP_LEDGER_CANISTER_ID }; 
    data.ogy_ledger_canister_id = if args.test_mode { OGY_LEDGER_CANISTER_ID_STAGING } else { OGY_LEDGER_CANISTER_ID }; 
    data.gldgov_ledger_canister_id = if args.test_mode { SNS_LEDGER_CANISTER_ID_STAGING } else { SNS_LEDGER_CANISTER_ID }; 

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
