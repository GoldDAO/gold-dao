use candid::CandidType;
use ic_cdk_macros::init;
use serde::Deserialize;
use tracing::info;
use types::{ TokenInfo, TokenSymbol };
use utils::{
    consts::{
        OGY_LEDGER_CANISTER_ID_STAGING,
        ICP_LEDGER_CANISTER_ID_STAGING,
        ICP_LEDGER_CANISTER_ID,
        OGY_LEDGER_CANISTER_ID,
        SNS_LEDGER_CANISTER_ID,
        SNS_LEDGER_CANISTER_ID_STAGING,
    },
    env::CanisterEnv,
};

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
    let icp_ledger_canister_id = if args.test_mode {
        ICP_LEDGER_CANISTER_ID_STAGING
    } else {
        ICP_LEDGER_CANISTER_ID
    };
    let ogy_ledger_canister_id = if args.test_mode {
        OGY_LEDGER_CANISTER_ID_STAGING
    } else {
        OGY_LEDGER_CANISTER_ID
    };
    let gldgov_ledger_canister_id = if args.test_mode {
        SNS_LEDGER_CANISTER_ID_STAGING
    } else {
        SNS_LEDGER_CANISTER_ID
    };

    if let Ok(token) = TokenSymbol::parse("ICP") {
        data.tokens.insert(token, TokenInfo {
            ledger_id: icp_ledger_canister_id,
            fee: 10_000u64,
            decimals: 8u64,
        });
    }
    if let Ok(token) = TokenSymbol::parse("OGY") {
        data.tokens.insert(token, TokenInfo {
            ledger_id: ogy_ledger_canister_id,
            fee: 200_000u64,
            decimals: 8u64,
        });
    }
    if let Ok(token) = TokenSymbol::parse("GLDGov") {
        data.tokens.insert(token, TokenInfo {
            ledger_id: gldgov_ledger_canister_id,
            fee: 100_000u64,
            decimals: 8u64,
        });
    }

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
