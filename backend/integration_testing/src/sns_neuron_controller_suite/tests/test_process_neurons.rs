use crate::client::rewards::add_neuron_ownership;
use candid::{CandidType, Deserialize};
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::add_neuron_ownership::Response as AddNeuronOwnerShipResponse;
use std::time::Duration;

use crate::{
    client::icrc1::client::{balance_of, transfer},
    sns_neuron_controller_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_process_neurons_happy_path() {
    let mut test_env = default_test_setup();

    println!("test_env: {:#?}", test_env);

    let ogy_ledger_canister_id = test_env
        .token_ledgers
        .get("ogy_ledger_canister_id")
        .unwrap()
        .clone();
    let ogy_rewards_canister_id = test_env.ogy_rewards_canister_id;

    let initial_sns_rewards_balance = balance_of(
        &mut test_env.pic,
        ogy_ledger_canister_id,
        Account {
            owner: test_env.gldt_rewards_canister_id,
            subaccount: None,
        },
    );
    println!("balance: {:?}", initial_sns_rewards_balance);

    let user_1 = test_env.sns_neuron_controller_id;
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1)); // double check the data correct (user_1's hotkey is on the first neuron's permissions list)

    // ********************************
    // 1. add ownership (the rewards are distributed to the neuron owner)
    // ********************************
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_1,
        ogy_rewards_canister_id,
        &neuron_id_1.clone(),
    );
    tick_n_blocks(&test_env.pic, 10);
    match res {
        AddNeuronOwnerShipResponse::Ok(n_id) => assert_eq!(n_id, neuron_id_1),
        _ => {}
    }

    // ********************************
    // 2. simulate distribution - add reward to neuron
    // ********************************

    let neuron_account_1 = Account {
        owner: ogy_rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };

    // Transfer "rewards" to the neuron
    transfer(
        &mut test_env.pic,
        test_env.sns_governance_id,
        ogy_ledger_canister_id,
        None,
        neuron_account_1,
        300_000_000_000_000, // 10,000 OGY
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);

    test_env.pic.advance_time(Duration::from_secs(1 * 60 * 60));
    tick_n_blocks(&test_env.pic, 10);

    let current_sns_rewards_balance = balance_of(
        &mut test_env.pic,
        ogy_ledger_canister_id,
        Account {
            owner: test_env.gldt_rewards_canister_id,
            subaccount: None,
        },
    );
    println!(
        "balance of {:?}: {:?}",
        test_env.gldt_rewards_canister_id, current_sns_rewards_balance
    );

    assert!(initial_sns_rewards_balance < current_sns_rewards_balance);
}
