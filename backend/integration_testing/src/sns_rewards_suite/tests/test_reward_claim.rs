use candid::Nat;
use icrc_ledger_types::icrc1::account::Account;
use sns_rewards_api_canister::claim_rewards_batch::ClaimRewardError;
use sns_rewards_api_canister::claim_rewards_batch::ClaimRewardErrorType;
use sns_rewards_api_canister::claim_rewards_batch::{
    Args as ClaimRewardArgs, Response as ClaimRewardResponse,
};

use crate::client::rewards::claim_rewards_batch;
use crate::{
    client::icrc1::client::{balance_of, transfer},
    sns_rewards_suite::setup::{default_test_setup, test_setup_with_no_neuron_hotkeys},
    utils::{random_principal, tick_n_blocks},
};

fn is_claim_rewards_batch_failed(value: &ClaimRewardResponse) -> bool {
    matches!(value, Err(_))
}

fn is_claim_rewards_batch_success(value: &ClaimRewardResponse) -> bool {
    matches!(value, ClaimRewardResponse::Ok(_))
}

#[test]
fn test_reward_claim_happy_path() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1)); // double check the data correct ( user_1's hotkey is on the first neuron's permissions list )

    // ********************************
    // 1. simulate distribution - add reward to neuron
    // ********************************
    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into(),
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // ********************************
    // 3. claim reward - as user_1
    // ********************************
    let res = claim_rewards_batch(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: vec![neuron_id_1.clone()],
            tokens: vec![types::TokenSymbol::ICP],
        }),
    );
    tick_n_blocks(&test_env.pic, 20);
    assert!(is_claim_rewards_batch_success(&res));

    // ********************************
    // 4. Check user got the correct reward
    // ********************************
    let user_1_account = Account {
        owner: user_1.clone(),
        subaccount: None,
    };
    let user_1_icp_balance = balance_of(&test_env.pic, icp_ledger_id, user_1_account);
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(
        user_1_icp_balance,
        Nat::from(100_000_000_00u64) - Nat::from(10_000u64)
    );
    tick_n_blocks(&test_env.pic, 20);
}

#[test]
fn test_reward_batch_claim_happy_path() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let ogy_ledger_id = test_env
        .token_ledgers
        .get("ogy_ledger_canister_id")
        .unwrap()
        .clone();
    let goldao_ledger_id = test_env
        .token_ledgers
        .get("goldao_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1)); // double check the data correct ( user_1's hotkey is on the first neuron's permissions list )

    // ********************************
    // 1. simulate distribution - add reward to neuron
    // ********************************
    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into(),
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        ogy_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into(),
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        goldao_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into(),
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // ********************************
    // 3. claim reward - as user_1
    // ********************************
    let res = claim_rewards_batch(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: vec![neuron_id_1.clone()],
            tokens: vec![
                types::TokenSymbol::ICP,
                types::TokenSymbol::OGY,
                types::TokenSymbol::GOLDAO,
            ],
        }),
    );
    tick_n_blocks(&test_env.pic, 20);
    assert!(is_claim_rewards_batch_success(&res));

    // ********************************
    // 4. Check user got the correct reward
    // ********************************
    let user_1_account = Account {
        owner: user_1.clone(),
        subaccount: None,
    };
    let user_1_icp_balance = balance_of(&test_env.pic, icp_ledger_id, user_1_account);
    let user_1_ogy_balance = balance_of(&test_env.pic, ogy_ledger_id, user_1_account);
    let user_1_goldao_balance = balance_of(&test_env.pic, goldao_ledger_id, user_1_account);
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(
        user_1_icp_balance,
        Nat::from(100_000_000_00u64) - Nat::from(10_000u64)
    );
    assert_eq!(
        user_1_ogy_balance,
        Nat::from(100_000_000_00u64) - Nat::from(200_000u64)
    );
    assert_eq!(
        user_1_goldao_balance,
        Nat::from(100_000_000_00u64) - Nat::from(100_000u64)
    );
}

#[test]
fn test_reward_batch_claim_partial_tokens() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let ogy_ledger_id = test_env
        .token_ledgers
        .get("ogy_ledger_canister_id")
        .unwrap()
        .clone();
    let goldao_ledger_id = test_env
        .token_ledgers
        .get("goldao_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    // ********************************
    // 1. simulate distribution - add reward to neuron (only ICP and OGY, no GOLDAO)
    // ********************************
    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (75_000_000_00u64).into(),
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        ogy_ledger_id,
        None,
        neuron_account_1,
        (125_000_000_00u64).into(),
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // ********************************
    // 3. claim reward - as user_1 (request ICP and OGY only)
    // ********************************
    let res = claim_rewards_batch(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: vec![neuron_id_1.clone()],
            tokens: vec![types::TokenSymbol::ICP, types::TokenSymbol::OGY],
        }),
    );
    tick_n_blocks(&test_env.pic, 20);
    assert!(is_claim_rewards_batch_success(&res));

    // ********************************
    // 4. Check user got the correct reward
    // ********************************
    let user_1_account = Account {
        owner: user_1.clone(),
        subaccount: None,
    };
    let user_1_icp_balance = balance_of(&test_env.pic, icp_ledger_id, user_1_account);
    let user_1_ogy_balance = balance_of(&test_env.pic, ogy_ledger_id, user_1_account);
    let user_1_goldao_balance = balance_of(&test_env.pic, goldao_ledger_id, user_1_account);
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(
        user_1_icp_balance,
        Nat::from(75_000_000_00u64) - Nat::from(10_000u64)
    );
    assert_eq!(
        user_1_ogy_balance,
        Nat::from(125_000_000_00u64) - Nat::from(200_000u64)
    );
    // GOLDAO balance should be 0 since no rewards were distributed
    assert_eq!(user_1_goldao_balance, Nat::from(0u64));
}

#[test]
fn test_reward_batch_claim_empty_tokens_list() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    // ********************************
    // 1. simulate distribution - add reward to neuron
    // ********************************
    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into(),
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // ********************************
    // 3. claim reward - as user_1 with empty tokens list
    // ********************************
    let res = claim_rewards_batch(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: vec![neuron_id_1.clone()],
            tokens: vec![],
        }),
    );
    tick_n_blocks(&test_env.pic, 20);
    assert!(is_claim_rewards_batch_success(&res));

    // ********************************
    // 4. Check user balance unchanged
    // ********************************
    let user_1_account = Account {
        owner: user_1.clone(),
        subaccount: None,
    };
    let user_1_icp_balance = balance_of(&test_env.pic, icp_ledger_id, user_1_account);
    tick_n_blocks(&test_env.pic, 10);
    // Should be 0 or minimal since no tokens were claimed
    assert_eq!(user_1_icp_balance, Nat::from(0u64));
}

#[test]
fn test_reward_batch_claim_different_amounts() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let ogy_ledger_id = test_env
        .token_ledgers
        .get("ogy_ledger_canister_id")
        .unwrap()
        .clone();
    let goldao_ledger_id = test_env
        .token_ledgers
        .get("goldao_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    // ********************************
    // 1. simulate distribution - add different reward amounts
    // ********************************
    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (200_000_000_00u64).into(),
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        ogy_ledger_id,
        None,
        neuron_account_1,
        (50_000_000_00u64).into(),
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        goldao_ledger_id,
        None,
        neuron_account_1,
        (300_000_000_00u64).into(),
    )
    .unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // ********************************
    // 3. claim reward - as user_1
    // ********************************
    let res = claim_rewards_batch(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: vec![neuron_id_1.clone()],
            tokens: vec![
                types::TokenSymbol::ICP,
                types::TokenSymbol::OGY,
                types::TokenSymbol::GOLDAO,
            ],
        }),
    );
    tick_n_blocks(&test_env.pic, 20);
    assert!(is_claim_rewards_batch_success(&res));

    // ********************************
    // 4. Check user got the correct reward
    // ********************************
    let user_1_account = Account {
        owner: user_1.clone(),
        subaccount: None,
    };
    let user_1_icp_balance = balance_of(&test_env.pic, icp_ledger_id, user_1_account);
    let user_1_ogy_balance = balance_of(&test_env.pic, ogy_ledger_id, user_1_account);
    let user_1_goldao_balance = balance_of(&test_env.pic, goldao_ledger_id, user_1_account);
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(
        user_1_icp_balance,
        Nat::from(200_000_000_00u64) - Nat::from(10_000u64)
    );
    assert_eq!(
        user_1_ogy_balance,
        Nat::from(50_000_000_00u64) - Nat::from(200_000u64)
    );
    assert_eq!(
        user_1_goldao_balance,
        Nat::from(300_000_000_00u64) - Nat::from(100_000u64)
    );
}

#[test]
fn test_neuron_with_no_hotkey() {
    let mut test_env = test_setup_with_no_neuron_hotkeys(); // every neuron has no hotkey

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    // let random_principal = Principal::anonymous();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone(); // has no hotkey
    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();
    assert!(neuron_1.permissions.get(1) == None); // should be no hotkey on this neuron

    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };

    // ********************************
    // 1. Claim reward as user 1 - SHOULD FAIL ( random principal not owner of neuron )
    // ********************************
    // add some rewards to claim just incase.
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into(),
    )
    .unwrap();

    let res = claim_rewards_batch(
        &mut test_env.pic,
        random_principal(),
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: vec![neuron_id_1.clone()],
            tokens: vec![types::TokenSymbol::ICP],
        }),
    );

    // ********************************
    // 4. Claim reward as neuron_1 owner principal - SHOULD PASS ( as it does own the neuron and is a hotkey )
    // ********************************
    let res = claim_rewards_batch(
        &mut test_env.pic,
        neuron_1.permissions.get(0).unwrap().principal.unwrap(),
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: vec![neuron_id_1.clone()],
            tokens: vec![types::TokenSymbol::ICP],
        }),
    );
    assert_eq!(res, ClaimRewardResponse::Ok(()));
}

#[test]
fn test_claim_rewards_batch_failures() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let user_2 = test_env.users.get(1).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };

    // ********************************
    // 1. Simulate distribution - Transfer some rewards to neuron
    // ********************************
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into(),
    )
    .unwrap();

    // ********************************
    // 1. Claim reward as user 2 - Should fail because user_2's hotkey is not on the neuron and they don't own it.
    // ********************************
    let res = claim_rewards_batch(
        &mut test_env.pic,
        user_2,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: vec![neuron_id_1.clone()],
            tokens: vec![types::TokenSymbol::ICP],
        }),
    );
    assert!(res.is_err_and(|err| {
        err.iter()
            .any(|e| matches!(e.error, ClaimRewardErrorType::NeuronHotKeyInvalid))
    }));
}

#[test]
fn test_claim_rewards_batch_fails_if_there_are_no_rewards() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };

    // ********************************
    // 1. Claim reward as user_1 - SHOULD FAIL ( no rewards to claim )
    // ********************************

    let res = claim_rewards_batch(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: vec![neuron_id_1.clone()],
            tokens: vec![types::TokenSymbol::ICP],
        }),
    );
    assert!(is_claim_rewards_batch_failed(&res));

    // ********************************
    // 1. Claim reward as user_1 - SHOULD FAIL ( not enough rewards to cover the transaction fees )
    // ********************************
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (5_000u64).into(),
    )
    .unwrap();
    // claim the reward - should fail because the fee is set to 10_000
    let res = claim_rewards_batch(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: vec![neuron_id_1.clone()],
            tokens: vec![types::TokenSymbol::ICP],
        }),
    );
    assert!(is_claim_rewards_batch_failed(&res));
}
