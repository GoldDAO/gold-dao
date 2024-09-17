use crate::client::rewards::add_neuron_ownership;
use candid::{ CandidType, Deserialize };
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::add_neuron_ownership::Response as AddNeuronOwnerShipResponse;
use std::time::Duration;
use crate::wasms;
use candid::encode_one;
use types::BuildVersion;

use crate::{
    client::icrc1::client::{ balance_of, transfer },
    versioning_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};
use crate::versioning_suite::setup::setup::InitArgs;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Args {
    Init(InitArgs),
    Upgrade(UpgradeArgs),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UpgradeArgs {
    pub wasm_version: BuildVersion,
    pub commit_hash: String,
}

#[test]
fn test_version_upgrade() {
    let mut test_env = default_test_setup();
    let controller = test_env.controller.clone();

    let upgrade_args = Args::Upgrade(UpgradeArgs {
        wasm_version: BuildVersion::min(),
        commit_hash: "TestCommitHash2".to_string(),
    });

    // Upgrade all the canisters
    test_env.pic
        .upgrade_canister(
            test_env.buyback_burn_canister_id,
            wasms::BUYBACK_BURN.clone(),
            encode_one(&upgrade_args).unwrap(),
            Some(controller.clone())
        )
        .unwrap();

    test_env.pic
        .upgrade_canister(
            test_env.icp_neuron_canister_id,
            wasms::ICP_NEURON.clone(),
            encode_one(&upgrade_args).unwrap(),
            Some(controller.clone())
        )
        .unwrap();
    test_env.pic
        .upgrade_canister(
            test_env.management_canister_id,
            wasms::MANAGEMENT.clone(),
            encode_one(&upgrade_args).unwrap(),
            Some(controller.clone())
        )
        .unwrap();
    test_env.pic
        .upgrade_canister(
            test_env.sns_neuron_controller_canister_id,
            wasms::SNS_NEURON_CONTROLLER.clone(),
            encode_one(&upgrade_args).unwrap(),
            Some(controller.clone())
        )
        .unwrap();
    test_env.pic
        .upgrade_canister(
            test_env.sns_rewards_canister_id,
            wasms::REWARDS.clone(),
            encode_one(&upgrade_args).unwrap(),
            Some(controller.clone())
        )
        .unwrap();
    test_env.pic
        .upgrade_canister(
            test_env.super_stats_v3_canister_id,
            wasms::SUPER_STATS.clone(),
            encode_one(&upgrade_args).unwrap(),
            Some(controller.clone())
        )
        .unwrap();
    test_env.pic
        .upgrade_canister(
            test_env.token_metrics_canister_id,
            wasms::TOKEN_METRICS.clone(),
            encode_one(&upgrade_args).unwrap(),
            Some(controller.clone())
        )
        .unwrap();
}
