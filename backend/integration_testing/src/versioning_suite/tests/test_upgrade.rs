use crate::wasms;
use candid::encode_one;
use types::BuildVersion;
use crate::utils::tick_n_blocks;

use crate::versioning_suite::setup::default_test_setup;
use crate::versioning_suite::setup::setup::*;

#[test]
fn test_version_upgrade() {
    let test_env = default_test_setup();
    let controller = test_env.controller.clone();

    let upgrade_args = Args::Upgrade(UpgradeArgs {
        version: BuildVersion::min(),
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

    println!("buyback_burn_canister upgraded");

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
