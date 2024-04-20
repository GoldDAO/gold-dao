use std::collections::HashMap;

use candid::{ encode_one, CandidType, Principal };
use pocket_ic::PocketIc;
use serde::Deserialize;

use crate::{ wasms, CanisterIds };

#[derive(Deserialize, CandidType)]
pub struct Args {
    test_mode: bool,
    icp_ledger_canister_id: Principal,
    sns_ledger_canister_id: Principal,
    ogy_ledger_canister_id: Principal,
    sns_gov_canister_id: Principal,
}

pub fn setup_rewards_canister(
    pic: &mut PocketIc,
    token_ledgers: &CanisterIds,
    sns_canister_id: &Principal
) -> Principal {
    let sns_subnet = pic.topology().get_sns().unwrap();
    let rewards_canister = pic.create_canister_on_subnet(None, None, sns_subnet);
    let rewards_wasm = wasms::REWARDS.clone();
    pic.add_cycles(rewards_canister, 1_000_000_000_000);

    let init_args = Args {
        test_mode: true,
        icp_ledger_canister_id: token_ledgers.icp_ledger_id,
        sns_ledger_canister_id: token_ledgers.gldgov_ledger_id,
        ogy_ledger_canister_id: token_ledgers.ogy_ledger_id,
        sns_gov_canister_id: sns_canister_id.clone(),
    };
    pic.install_canister(rewards_canister, rewards_wasm, encode_one(init_args).unwrap(), None);
    rewards_canister
}

pub fn setup_rewards_canister_v2(
    pic: &mut PocketIc,
    token_ledgers: &HashMap<String, Principal>,
    sns_canister_id: &Principal
) -> Principal {
    let sns_subnet = pic.topology().get_sns().unwrap();
    let rewards_canister = pic.create_canister_on_subnet(None, None, sns_subnet);
    let rewards_wasm = wasms::REWARDS.clone();
    pic.add_cycles(rewards_canister, 1_000_000_000_000);

    let icp_ledger_canister_id = token_ledgers
        .get("icp_ledger_canister_id")
        .expect("couldn't find ledger with 'icp_ledger_canister_id'")
        .clone();
    let sns_ledger_canister_id = token_ledgers
        .get("gldgov_ledger_canister_id")
        .expect("couldn't find ledger with 'gldgov_ledger_canister_id'")
        .clone();
    let ogy_ledger_canister_id = token_ledgers
        .get("ogy_ledger_canister_id")
        .expect("couldn't find ledger with 'ogy_ledger_canister_id'")
        .clone();

    let init_args = Args {
        test_mode: true,
        icp_ledger_canister_id,
        sns_ledger_canister_id,
        ogy_ledger_canister_id,
        sns_gov_canister_id: sns_canister_id.clone(),
    };
    pic.install_canister(rewards_canister, rewards_wasm, encode_one(init_args).unwrap(), None);
    rewards_canister
}
