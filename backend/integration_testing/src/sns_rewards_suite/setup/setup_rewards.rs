use std::collections::HashMap;

use candid::{encode_one, Principal};
use pocket_ic::PocketIc;
use sns_rewards_api_canister::init::InitArgs;
use sns_rewards_api_canister::post_upgrade::UpgradeArgs;
use sns_rewards_api_canister::Args;
use types::BuildVersion;

use crate::wasms;

pub fn setup_rewards_canister(
    pic: &PocketIc,
    token_ledgers: &HashMap<String, Principal>,
    sns_canister_id: &Principal,
    controller: &Principal,
) -> Principal {
    let sns_subnet = pic.topology().get_sns().unwrap();
    let rewards_canister =
        pic.create_canister_on_subnet(Some(controller.clone()), None, sns_subnet);

    let rewards_wasm = wasms::REWARDS.clone();
    pic.add_cycles(rewards_canister, 100_000_000_000_000_000);
    pic.set_controllers(
        rewards_canister,
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
        .get("goldao_ledger_canister_id")
        .expect("couldn't find ledger with 'goldao_ledger_canister_id'")
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
        rewards_canister,
        rewards_wasm,
        encode_one(init_args).unwrap(),
        Some(controller.clone()),
    );
    rewards_canister
}

use pocket_ic::RejectResponse;
pub fn upgrade_rewards_canister(
    pic: &PocketIc,
    canister_id: Principal,
    controller: &Principal,
) -> std::result::Result<(), RejectResponse> {
    let rewards_wasm = wasms::REWARDS.clone();

    let upgrade_args = Args::Upgrade(UpgradeArgs {
        version: BuildVersion::min(),
        commit_hash: "Test".to_string(),
    });

    let encoded_args = encode_one(upgrade_args).expect("Failed to encode upgrade args");

    pic.upgrade_canister(
        canister_id,
        rewards_wasm,
        encoded_args,
        Some(controller.clone()),
    )
}
