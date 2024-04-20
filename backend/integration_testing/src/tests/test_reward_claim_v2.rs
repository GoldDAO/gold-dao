use candid::{ CandidType, Deserialize, Nat, Principal };
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use sns_rewards::types::claim_neuron_response::UserClaimErrorResponse;

use crate::{
    client::{
        icrc1::client::{ balance_of, transfer },
        pocket::execute_update_multi_args,
        rewards::{ add_neuron_ownership, remove_neuron_ownership },
    },
    setup::setup::{ default_test_setup, init, test_setup_with_no_neuron_hotkeys, TestEnv },
    utils::{ hex_to_subaccount, tick_n_blocks },
};

fn is_transaction_fail_enum(value: &UserClaimErrorResponse) -> bool {
    matches!(value, UserClaimErrorResponse::TransferFailed(_))
}

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_reward_claim_happy_path_v2() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let ogy_ledger_id = test_env.token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1)); // double check the data correct ( user_1's hotkey is on the first neuron's permissions list )

    // simulate a distribution by add some ICP rewards to a neuron that is owned by user_1 - see sns.rs for which neurons have users as hotkeys
    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    transfer(
        &mut test_env.pic,
        controller,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // add ownership - should return ok because user_1 has their hotkey on the neuron
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, neuron_id_1.clone());

    // claim the reward - should return true
    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    ).unwrap();
    tick_n_blocks(&test_env.pic, 20);
    assert_eq!(res, true);

    // check the balance to verify the reward - fee exists
    let user_1_account = Account {
        owner: user_1.clone(),
        subaccount: None,
    };
    let user_1_icp_balance = balance_of(&test_env.pic, icp_ledger_id, user_1_account);
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(user_1_icp_balance, Nat::from(100_000_000_00u64) - Nat::from(10_000u64));
}

#[test]
fn test_add_neuron_ownership_failures_v2() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let ogy_ledger_id = test_env.token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let user_2 = test_env.users.get(1).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    transfer(
        &mut test_env.pic,
        controller,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // add ownership - should error because user_1 has a hotkey on the neuron but user_2 called
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_2,
        rewards_canister_id,
        &neuron_id_1.clone()
    )
        .err()
        .unwrap();
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyInvalid);

    // should fail if the neuron doesn't exist in the sns
    let non_exitent_neuron = &NeuronId::new(
        "5129ea7ec019c2a5f19b16ae3562870556b6f4cb424496f6255215a33465eb21"
    ).unwrap();
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_2,
        rewards_canister_id,
        &non_exitent_neuron.clone()
    )
        .err()
        .unwrap();
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, UserClaimErrorResponse::NeuronDoesNotExist);
}

#[test]
fn test_remove_neuron_ownership_failures_v2() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let ogy_ledger_id = test_env.token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let user_2 = test_env.users.get(1).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    // user_1 has ownership
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, neuron_id_1.clone());

    // try to remove ownership as user 2
    let res = remove_neuron_ownership(
        &mut test_env.pic,
        user_2,
        rewards_canister_id,
        &neuron_id_1.clone()
    )
        .err()
        .unwrap();
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyInvalid);

    // remove neuron as user 1 - should be ok
    let res = remove_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    ).unwrap();
    assert_eq!(res, neuron_id_1.clone());
}

#[test]
fn test_neuron_with_no_hotkey_v2() {
    let mut test_env = test_setup_with_no_neuron_hotkeys(); // every neuron has no hotkey

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let ogy_ledger_id = test_env.token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let random_principal = Principal::anonymous();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone(); // has no hotkey
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1) == None); // should be no hotkey on this neuron

    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };

    // try to add user_1 as owner - should fail because there are no hotkeys on the neuron
    let res = add_neuron_ownership(
        &mut test_env.pic,
        random_principal,
        rewards_canister_id,
        &neuron_id_1.clone()
    )
        .err()
        .unwrap();
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyAbsent);

    // try to remove neuron - should fail because there are no hotkeys on the neuron
    let res = remove_neuron_ownership(
        &mut test_env.pic,
        random_principal,
        rewards_canister_id,
        &neuron_id_1.clone()
    )
        .err()
        .unwrap();
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyAbsent);

    // test claiming a neuron's rewards with no hotkey
    transfer(
        &mut test_env.pic,
        controller,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);

    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut test_env.pic,
        random_principal,
        rewards_canister_id,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    )
        .err()
        .unwrap();
    tick_n_blocks(&test_env.pic, 20);
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyAbsent);
}

#[test]
fn test_claim_reward_failures_v2() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let ogy_ledger_id = test_env.token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let user_2 = test_env.users.get(1).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    transfer(
        &mut test_env.pic,
        controller,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // add ownership - should return ok
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, neuron_id_1.clone());

    // claim reward - should fail because neuron_1 has hotkey and ownership but user_2 called
    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut test_env.pic,
        user_2,
        rewards_canister_id,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    )
        .err()
        .unwrap();
    tick_n_blocks(&test_env.pic, 20);
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyInvalid);
}

#[test]
fn test_claim_reward_fails_if_there_are_no_rewards_v2() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let ogy_ledger_id = test_env.token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let user_2 = test_env.users.get(1).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };

    // add ownership - should return ok because user_1 has their hotkey on the neuron
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, neuron_id_1.clone());

    // claim the reward - should fail because there are no rewards to claim
    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    )
        .err()
        .unwrap();
    tick_n_blocks(&test_env.pic, 20);
    assert!(is_transaction_fail_enum(&res));

    // add 5000 as rewards
    transfer(
        &mut test_env.pic,
        controller,
        icp_ledger_id,
        None,
        neuron_account_1,
        (5_000u64).into()
    ).unwrap();
    // claim the reward - should fail because the fee is set to 10_000
    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    )
        .err()
        .unwrap();
    tick_n_blocks(&test_env.pic, 20);
    assert!(is_transaction_fail_enum(&res));
}
