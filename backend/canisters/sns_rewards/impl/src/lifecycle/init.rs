use candid::{ CandidType, Nat };
use ic_cdk_macros::init;
use serde::Deserialize;
use tracing::info;
use types::{ TokenInfo, TokenSymbol };
use utils::{
    consts::{
        ICP_LEDGER_CANISTER_ID_STAGING,
        OGY_LEDGER_CANISTER_ID_STAGING,
        SNS_GOVERNANCE_CANISTER_ID_STAGING,
        SNS_GOVERNANCE_CANISTER_ID_TESTING,
        SNS_LEDGER_CANISTER_ID_STAGING,
    },
    env::CanisterEnv,
};

use crate::state::{ Data, RuntimeState };

use super::init_canister;

#[derive(Deserialize, CandidType)]
pub struct Args {
    test_mode: bool,
    pocket_ic: bool,
}

#[init]
fn init(args: Args) {
    canister_logger::init(args.test_mode);

    let env = CanisterEnv::new(args.test_mode);
    let mut data = Data::default();

    // use staging canister ids
    if args.test_mode {
        let icp_ledger_canister_id = ICP_LEDGER_CANISTER_ID_STAGING;
        let ogy_ledger_canister_id = OGY_LEDGER_CANISTER_ID_STAGING;
        let gldgov_ledger_canister_id = SNS_LEDGER_CANISTER_ID_STAGING;

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
            data.tokens.insert(token.clone(), TokenInfo {
                ledger_id: gldgov_ledger_canister_id,
                fee: 100_000u64,
                decimals: 8u64,
            });
            data.daily_reserve_transfer.insert(token, Nat::from(10000u64));
        }

        data.sns_governance_canister = SNS_GOVERNANCE_CANISTER_ID_STAGING;
    }

    if args.pocket_ic {
        data.sns_governance_canister = SNS_GOVERNANCE_CANISTER_ID_TESTING;
    }

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
