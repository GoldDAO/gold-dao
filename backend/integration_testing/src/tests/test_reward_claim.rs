use std::{ borrow::BorrowMut, thread, time::Duration };

use candid::{ CandidType, Deserialize, Nat, Principal };
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::PocketIc;
use serde::Serialize;
use serde_bytes::ByteBuf;
use sns_governance_canister::types::{ neuron, NeuronId };
use sns_rewards::{
    consts::{ RESERVE_POOL_SUB_ACCOUNT, REWARD_POOL_SUB_ACCOUNT },
    types::claim_neuron_response::UserClaimErrorResponse,
};

use crate::{
    client::{
        icrc1::happy_path::{ balance_of, transfer },
        pocket::execute_update_multi_args,
        rewards::{
            add_neuron_ownership,
            claim_reward,
            get_all_neurons,
            get_neuron_by_id,
            http_request,
            remove_neuron_ownership,
            sync_neurons_manual_trigger,
        },
    },
    setup::{ setup::{ init, TestEnv }, sns::{ generate_neuron_data_for_week, setup_sns_by_week } },
    utils::{ decode_http_bytes, hex_to_subaccount, tick_n_blocks },
};

fn is_transaction_fail_enum(value: &UserClaimErrorResponse) -> bool {
    matches!(value, UserClaimErrorResponse::TransferFailed(_))
}

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_reward_claim_happy_path() {
    let env = init();
    let TestEnv { mut pic, controller, token_ledgers, mut sns, rewards } = env;
    let sns_gov_id = sns.sns_gov_id.clone();

    let user_1 = sns.users.get(0).unwrap().clone();
    let user_2 = sns.users.get(1).unwrap().clone();

    // simulate a distribution by add some ICP rewards to a neuron that is owned by user_1 - see sns.rs for which neurons have users as hotkeys
    let neuron_id_1 = &NeuronId::new(
        "5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21"
    ).unwrap();
    let neuron_account_1 = Account {
        owner: rewards,
        subaccount: Some(
            hex_to_subaccount("5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21")
        ),
    };
    transfer(
        &mut pic,
        controller,
        token_ledgers.icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&pic, 10);

    // add ownership - should return ok because user_1 has their hotkey on the neuron
    let res = add_neuron_ownership(&mut pic, user_1, rewards, &neuron_id_1.clone()).unwrap();
    tick_n_blocks(&pic, 10);
    assert_eq!(res, neuron_id_1.clone());

    // claim the reward - should return true
    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut pic,
        user_1,
        rewards,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    ).unwrap();
    tick_n_blocks(&pic, 20);
    assert_eq!(res, true);

    // check the balance to verify the reward - fee exists
    let user_1_account = Account {
        owner: user_1.clone(),
        subaccount: None,
    };
    let user_1_icp_balance = balance_of(&pic, token_ledgers.icp_ledger_id, user_1_account);
    tick_n_blocks(&pic, 10);
    assert_eq!(user_1_icp_balance, Nat::from(100_000_000_00u64) - Nat::from(10_000u64));
}

#[test]
fn test_add_neuron_ownership_failures() {
    let env = init();
    let TestEnv { mut pic, controller, token_ledgers, mut sns, rewards } = env;
    let sns_gov_id = sns.sns_gov_id.clone();

    let user_1 = sns.users.get(0).unwrap().clone();
    let user_2 = sns.users.get(1).unwrap().clone();

    // simulate a distribution by add some ICP rewards to a neuron that is owned by user_1 - see sns.rs for which neurons have users as hotkeys
    let neuron_id_1 = &NeuronId::new(
        "5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21"
    ).unwrap();
    let neuron_account_1 = Account {
        owner: rewards,
        subaccount: Some(
            hex_to_subaccount("5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21")
        ),
    };
    transfer(
        &mut pic,
        controller,
        token_ledgers.icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&pic, 10);

    // add ownership - should error because user_1 has a hotkey on the neuron but user_2 called
    let res = add_neuron_ownership(&mut pic, user_2, rewards, &neuron_id_1.clone()).err().unwrap();
    tick_n_blocks(&pic, 10);
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyInvalid);

    // should fail if the neuron doesn't exist in the sns
    let non_exitent_neuron = &NeuronId::new(
        "5129ea7ec019c2a5f19b16ae3562870556b6f4cb424496f6255215a33465eb21"
    ).unwrap();
    let res = add_neuron_ownership(&mut pic, user_2, rewards, &non_exitent_neuron.clone())
        .err()
        .unwrap();
    tick_n_blocks(&pic, 10);
    assert_eq!(res, UserClaimErrorResponse::NeuronDoesNotExist);
}

#[test]
fn test_remove_neuron_ownership_failures() {
    let env = init();
    let TestEnv { mut pic, controller, token_ledgers, mut sns, rewards } = env;
    let sns_gov_id = sns.sns_gov_id.clone();

    let user_1 = sns.users.get(0).unwrap().clone();
    let user_2 = sns.users.get(1).unwrap().clone();

    // simulate a distribution by add some ICP rewards to a neuron that is owned by user_1 - see sns.rs for which neurons have users as hotkeys
    let neuron_id_1 = &NeuronId::new(
        "5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21"
    ).unwrap();
    let neuron_account_1 = Account {
        owner: rewards,
        subaccount: Some(
            hex_to_subaccount("5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21")
        ),
    };

    // user_1 has ownership
    let res = add_neuron_ownership(&mut pic, user_1, rewards, &neuron_id_1.clone()).unwrap();
    tick_n_blocks(&pic, 10);
    assert_eq!(res, neuron_id_1.clone());

    // try to remove ownership as user 2
    let res = remove_neuron_ownership(&mut pic, user_2, rewards, &neuron_id_1.clone())
        .err()
        .unwrap();
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyInvalid);

    // remove neuron as user 1 - should be ok
    let res = remove_neuron_ownership(&mut pic, user_1, rewards, &neuron_id_1.clone()).unwrap();
    assert_eq!(res, neuron_id_1.clone());
}

#[test]
fn test_neuron_with_no_hotkey() {
    let env = init();
    let TestEnv { mut pic, controller, token_ledgers, mut sns, rewards } = env;
    let sns_gov_id = sns.sns_gov_id.clone();

    let user_1 = sns.users.get(0).unwrap().clone();
    let user_2 = sns.users.get(1).unwrap().clone();

    // neuron with no hotkey
    let neuron_id_1 = &NeuronId::new(
        "9ac039bb64c76f1ff554eef3a4594a11d1611feaf9052e7ad8166461f997a12f"
    ).unwrap();
    let neuron_account_1 = Account {
        owner: rewards,
        subaccount: Some(
            hex_to_subaccount("9ac039bb64c76f1ff554eef3a4594a11d1611feaf9052e7ad8166461f997a12f")
        ),
    };

    // try to add user_1 as owner - should fail because there are no hotkeys on the neuron
    let res = add_neuron_ownership(&mut pic, user_1, rewards, &neuron_id_1.clone()).err().unwrap();
    tick_n_blocks(&pic, 10);
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyAbsent);

    // try to remove neuron - should fail because there are no hotkeys on the neuron
    let res = remove_neuron_ownership(&mut pic, user_1, rewards, &neuron_id_1.clone())
        .err()
        .unwrap();
    tick_n_blocks(&pic, 10);
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyAbsent);

    // test claiming a neuron's rewards with no hotkey
    transfer(
        &mut pic,
        controller,
        token_ledgers.icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&pic, 10);

    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut pic,
        user_1,
        rewards,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    )
        .err()
        .unwrap();
    tick_n_blocks(&pic, 20);
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyAbsent);
}

#[test]
fn test_claim_reward_failures() {
    let env = init();
    let TestEnv { mut pic, controller, token_ledgers, mut sns, rewards } = env;
    let sns_gov_id = sns.sns_gov_id.clone();

    let user_1 = sns.users.get(0).unwrap().clone();
    let user_2 = sns.users.get(1).unwrap().clone();

    // simulate a distribution by add some ICP rewards to a neuron that is owned by user_1 - see sns.rs for which neurons have users as hotkeys
    let neuron_id_1 = &NeuronId::new(
        "5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21"
    ).unwrap();
    let neuron_account_1 = Account {
        owner: rewards,
        subaccount: Some(
            hex_to_subaccount("5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21")
        ),
    };
    transfer(
        &mut pic,
        controller,
        token_ledgers.icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&pic, 10);

    // add ownership - should return ok
    let res = add_neuron_ownership(&mut pic, user_1, rewards, &neuron_id_1.clone()).unwrap();
    tick_n_blocks(&pic, 10);
    assert_eq!(res, neuron_id_1.clone());

    // claim reward - should fail because neuron_1 has hotkey and ownership but user_2 called
    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut pic,
        user_2,
        rewards,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    )
        .err()
        .unwrap();
    tick_n_blocks(&pic, 20);
    assert_eq!(res, UserClaimErrorResponse::NeuronHotKeyInvalid);
}

#[test]
fn test_claim_reward_fails_if_there_are_no_rewards() {
    let env = init();
    let TestEnv { mut pic, controller, token_ledgers, mut sns, rewards } = env;
    let sns_gov_id = sns.sns_gov_id.clone();

    let user_1 = sns.users.get(0).unwrap().clone();
    let user_2 = sns.users.get(1).unwrap().clone();

    // simulate a distribution by add some ICP rewards to a neuron that is owned by user_1 - see sns.rs for which neurons have users as hotkeys
    let neuron_id_1 = &NeuronId::new(
        "5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21"
    ).unwrap();
    let neuron_account_1 = Account {
        owner: rewards,
        subaccount: Some(
            hex_to_subaccount("5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21")
        ),
    };

    // add ownership - should return ok because user_1 has their hotkey on the neuron
    let res = add_neuron_ownership(&mut pic, user_1, rewards, &neuron_id_1.clone()).unwrap();
    tick_n_blocks(&pic, 10);
    assert_eq!(res, neuron_id_1.clone());

    // claim the reward - should fail because there are no rewards to claim
    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut pic,
        user_1,
        rewards,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    )
        .err()
        .unwrap();
    tick_n_blocks(&pic, 20);
    assert!(is_transaction_fail_enum(&res));

    // add 5000 as rewards
    transfer(
        &mut pic,
        controller,
        token_ledgers.icp_ledger_id,
        None,
        neuron_account_1,
        (5_000u64).into()
    ).unwrap();
    // claim the reward - should fail because the fee is set to 10_000
    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut pic,
        user_1,
        rewards,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    )
        .err()
        .unwrap();
    tick_n_blocks(&pic, 20);
    assert!(is_transaction_fail_enum(&res));
}
