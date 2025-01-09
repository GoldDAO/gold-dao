use std::collections::HashMap;

use candid::{encode_one, CandidType, Principal};
use pocket_ic::PocketIc;
use serde::Deserialize;
use sns_rewards_api_canister::init::InitArgs;
use sns_rewards_api_canister::Args;
use types::BuildVersion;

use crate::wasms;

pub fn setup_rewards_canister(
    pic: &mut PocketIc,
    sns_rewards_id: Principal,
    token_ledgers: &HashMap<String, Principal>,
    sns_canister_id: Principal,
    controller: &Principal,
) -> Principal {
    let rewards_wasm = wasms::REWARDS.clone();
    pic.add_cycles(sns_rewards_id, 100_000_000_000_000_000);
    pic.set_controllers(
        sns_rewards_id,
        Some(controller.clone()),
        vec![controller.clone()],
    )
    .unwrap();
    pic.tick();

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

    let init_args = Args::Init(InitArgs {
        test_mode: true,
        version: BuildVersion::min(),
        commit_hash: "Test".to_string(),
        icp_ledger_canister_id,
        sns_ledger_canister_id,
        ogy_ledger_canister_id,
        sns_gov_canister_id: sns_canister_id.clone(),
    });
    pic.install_canister(
        sns_rewards_id,
        rewards_wasm,
        encode_one(init_args).unwrap(),
        Some(controller.clone()),
    );
    sns_rewards_id
}
