use candid::{ CandidType, Nat, Principal };
use ic_cdk_macros::init;
use serde::Deserialize;
use tracing::info;
use types::{ TokenInfo, TokenSymbol };
use utils::env::CanisterEnv;

use crate::state::{ Data, RuntimeState };

use super::init_canister;

#[derive(Deserialize, CandidType)]
pub struct Args {
    test_mode: bool,
    icp_ledger_canister_id: Principal,
    sns_ledger_canister_id: Principal,
    ogy_ledger_canister_id: Principal,
    sns_gov_canister_id: Principal,
}

#[init]
fn init(args: Args) {
    canister_logger::init(args.test_mode);

    let env = CanisterEnv::new(args.test_mode);
    let mut data = Data::default();

    // use staging canister ids
    if args.test_mode {
        let icp_ledger_canister_id = args.icp_ledger_canister_id;
        let ogy_ledger_canister_id = args.ogy_ledger_canister_id;
        let gldgov_ledger_canister_id = args.sns_ledger_canister_id;

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
            data.daily_reserve_transfer.insert(token, Nat::from(100_000_000u64));
        }

        data.sns_governance_canister = args.sns_gov_canister_id;
    }

    let runtime_state = RuntimeState::new(env, data);

    init_canister(runtime_state);

    info!("Init complete.")
}
