use candid::{ CandidType, Deserialize, Nat, Principal };
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::{
    add_neuron_ownership::Response as AddNeuronOwnerShipResponse,
    remove_neuron_ownership::Response as RemoveNeuronOwnershipResponse,
    claim_reward::{ Args as ClaimRewardArgs, Response as ClaimRewardResponse },
};

use crate::{
    client::{
        icrc1::client::{ balance_of, transfer },
        rewards::{ add_neuron_ownership, claim_reward, remove_neuron_ownership },
    },
    setup::{ default_test_setup, test_setup_with_no_neuron_hotkeys },
    utils::tick_n_blocks,
};

fn is_claim_reward_fail(value: &ClaimRewardResponse) -> bool {
    matches!(value, ClaimRewardResponse::TransferFailed(_))
}

fn is_claim_reward_success(value: &ClaimRewardResponse) -> bool {
    matches!(value, ClaimRewardResponse::Ok(_))
}

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_reward_claim_happy_path() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
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
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // ********************************
    // 2. add ownership
    // ********************************
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    );
    println!("{res:?}");
    tick_n_blocks(&test_env.pic, 10);
    match res {
        AddNeuronOwnerShipResponse::Ok(n_id) => assert_eq!(n_id, neuron_id_1),
        _ => {}
    }

    // ********************************
    // 3. claim reward - as user_1
    // ********************************
    let res = claim_reward(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: neuron_id_1.clone(),
            token: "ICP".to_string(),
        })
    );
    tick_n_blocks(&test_env.pic, 20);
    assert!(is_claim_reward_success(&res));

    // ********************************
    // 4. Check user got the correct reward
    // ********************************
    let user_1_account = Account {
        owner: user_1.clone(),
        subaccount: None,
    };
    let user_1_icp_balance = balance_of(&test_env.pic, icp_ledger_id, user_1_account);
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(user_1_icp_balance, Nat::from(100_000_000_00u64) - Nat::from(10_000u64));
    tick_n_blocks(&test_env.pic, 20);
}

#[test]
fn test_add_neuron_ownership_failures() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let user_2 = test_env.users.get(1).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1)); // check user_1 is the owner in the generated data before starting

    // ********************************
    // 1. Distribute rewards - add rewards to neuron manually
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
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // ********************************
    // 2. Add ownership as user_2 - SHOULD FAIL
    // ********************************
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_2,
        rewards_canister_id,
        &neuron_id_1.clone()
    );
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, AddNeuronOwnerShipResponse::NeuronHotKeyInvalid);

    // ********************************
    // 1.b - Check adding neurons that don't exist
    // ********************************
    let non_exitent_neuron = &NeuronId::new(
        "5129ea7ec019c2a5f19b16ae3562870556b6f4cb424496f6255215a33465eb21"
    ).unwrap();
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_2,
        rewards_canister_id,
        &non_exitent_neuron.clone()
    );
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, AddNeuronOwnerShipResponse::NeuronDoesNotExist);
    tick_n_blocks(&test_env.pic, 20);
}

#[test]
fn test_remove_neuron_ownership_failures() {
    let mut test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let user_2 = test_env.users.get(1).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    // ********************************
    // 1. add neuron ownership to user_1
    // ********************************
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    );
    tick_n_blocks(&test_env.pic, 10);
    assert_eq!(res, AddNeuronOwnerShipResponse::Ok(neuron_id_1.clone()));

    // ********************************
    // 2. try to remove neuron ownership as user 2 - SHOULD FAIL
    // ********************************
    let res = remove_neuron_ownership(
        &mut test_env.pic,
        user_2,
        rewards_canister_id,
        &neuron_id_1.clone()
    );
    assert_eq!(res, RemoveNeuronOwnershipResponse::NeuronHotKeyInvalid);

    // ********************************
    // 3. remove ownership as user_1 ( owner ) - should succeed
    // ********************************
    let res = remove_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    );
    assert_eq!(res, RemoveNeuronOwnershipResponse::Ok(neuron_id_1.clone()));
    tick_n_blocks(&test_env.pic, 20);
}

#[test]
fn test_neuron_with_no_hotkey() {
    let mut test_env = test_setup_with_no_neuron_hotkeys(); // every neuron has no hotkey

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let random_principal = Principal::anonymous();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone(); // has no hotkey
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1) == None); // should be no hotkey on this neuron

    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };

    // ********************************
    // 1. Add neuron owner as user_1 - SHOULD FAIL ( NO hotkeys on neuron for any user )
    // ********************************

    let res = add_neuron_ownership(
        &mut test_env.pic,
        random_principal,
        rewards_canister_id,
        &neuron_id_1.clone()
    );
    assert_eq!(res, AddNeuronOwnerShipResponse::NeuronHotKeyAbsent);

    // ********************************
    // 1. remove owner as user_1 - SHOULD FAIL ( No hotkeys on neuron for any user )
    // ********************************

    let res = remove_neuron_ownership(
        &mut test_env.pic,
        random_principal,
        rewards_canister_id,
        &neuron_id_1.clone()
    );
    assert_eq!(res, RemoveNeuronOwnershipResponse::NeuronHotKeyAbsent);

    // ********************************
    // 1. Claim reward as user 1 - SHOULD FAIL ( no hotkeys on neuron for any user )
    // ********************************
    // add some rewards to claim just incase.
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();

    let res = claim_reward(
        &mut test_env.pic,
        random_principal,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: neuron_id_1.clone(),
            token: "ICP".to_string(),
        })
    );
    assert_eq!(res, ClaimRewardResponse::NeuronHotKeyAbsent);
}

#[test]
fn test_claim_reward_failures() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
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

    // ********************************
    // 1. Simulate distribution - Transfer some rewards to neuron
    // ********************************
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();

    // ********************************
    // 1. Add ownership as user 1 - Ok
    // ********************************
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    );
    assert_eq!(res, AddNeuronOwnerShipResponse::Ok(neuron_id_1.clone()));

    // ********************************
    // 1. Claim reward as user 2 - Should fail because user_2's hotkey is not on the neuron and they don't own it.
    // ********************************
    let res = claim_reward(
        &mut test_env.pic,
        user_2,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: neuron_id_1.clone(),
            token: "ICP".to_string(),
        })
    );
    assert_eq!(res, ClaimRewardResponse::NeuronHotKeyInvalid);
}

#[test]
fn test_claim_reward_fails_if_there_are_no_rewards() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1));

    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };

    // ********************************
    // 1. Add ownership as user_! - Ok
    // ********************************
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    );
    assert_eq!(res, AddNeuronOwnerShipResponse::Ok(neuron_id_1.clone()));

    // ********************************
    // 1. Claim reward as user_1 - SHOULD FAIL ( no rewards to claim )
    // ********************************

    let res = claim_reward(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: neuron_id_1.clone(),
            token: "ICP".to_string(),
        })
    );
    assert!(is_claim_reward_fail(&res));

    // ********************************
    // 1. Claim reward as user_1 - SHOULD FAIL ( not enough rewards to cover the transaction fees )
    // ********************************
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (5_000u64).into()
    ).unwrap();
    // claim the reward - should fail because the fee is set to 10_000
    let res = claim_reward(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &(ClaimRewardArgs {
            neuron_id: neuron_id_1.clone(),
            token: "ICP".to_string(),
        })
    );
    assert!(is_claim_reward_fail(&res));
}
