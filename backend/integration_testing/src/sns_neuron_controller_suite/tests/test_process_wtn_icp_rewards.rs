use crate::sns_neuron_controller_suite::setup::default_test_setup;
use crate::{
    client::icrc1::client::{balance_of, transfer},
    utils::tick_n_blocks,
};
use candid::{CandidType, Deserialize};
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use std::time::Duration;

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}
use candid::Nat;

#[test]
fn test_process_wtn_neurons_happy_path() {
    let test_env = default_test_setup();

    let icp_ledger_canister_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();

    // Transfer "rewards" to the canister
    transfer(
        &mut test_env.get_pic(),
        test_env.controller.clone(),
        icp_ledger_canister_id,
        None,
        test_env.sns_neuron_controller_id,
        100_000_000,
    )
    .unwrap();

    // NOTE: wait half of the day to skip the even of rewards distribution
    test_env
        .get_pic()
        .advance_time(Duration::from_secs(12 * 60));
    tick_n_blocks(&test_env.get_pic(), 1);

    let initial_icp_rewards_balance = balance_of(
        &mut test_env.get_pic(),
        icp_ledger_canister_id,
        Account {
            owner: test_env.sns_neuron_controller_id,
            subaccount: None,
        },
    );
    println!(
        "initial_icp_rewards_balance: {:?}",
        initial_icp_rewards_balance
    );

    test_env
        .get_pic()
        .advance_time(Duration::from_secs(24 * 60));
    tick_n_blocks(&test_env.get_pic(), 10);

    let initial_gld_rewards_canister_id_balance = balance_of(
        &mut test_env.get_pic(),
        icp_ledger_canister_id,
        test_env.gld_rewards_canister_id,
    );
    println!(
        "initial_gld_rewards_canister_id_balance: {:?}",
        initial_gld_rewards_canister_id_balance
    );

    test_env
        .get_pic()
        .advance_time(Duration::from_secs(24 * 60 * 60));
    tick_n_blocks(&test_env.get_pic(), 10);

    let current_icp_rewards_balance = balance_of(
        &mut test_env.get_pic(),
        icp_ledger_canister_id,
        test_env.sns_neuron_controller_id,
    );
    println!(
        "current_icp_rewards_balance: {:?}",
        current_icp_rewards_balance
    );

    let current_gld_rewards_canister_id_balance = balance_of(
        &mut test_env.get_pic(),
        icp_ledger_canister_id,
        test_env.gld_rewards_canister_id,
    );
    println!(
        "current_gld_rewards_canister_id_balance: {:?}",
        current_gld_rewards_canister_id_balance
    );

    assert!(initial_gld_rewards_canister_id_balance < current_gld_rewards_canister_id_balance);
    assert!(initial_icp_rewards_balance > current_icp_rewards_balance);

    // Should be 0 as all were claimed
    assert_eq!(current_icp_rewards_balance, Nat::from(0u8));
    // Should be the initial balance - 1x fee as one transaction happens in the distribution process.
    assert_eq!(
        current_gld_rewards_canister_id_balance,
        initial_icp_rewards_balance - Nat::from(10_000u32)
    );
}
