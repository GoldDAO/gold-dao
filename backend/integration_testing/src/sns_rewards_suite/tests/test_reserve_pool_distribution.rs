use std::{collections::HashMap, time::Duration};

use candid::{Nat, Principal};
use canister_time::DAY_IN_MS;
use icrc_ledger_types::icrc1::account::Account;

use sns_rewards_api_canister::subaccounts::{RESERVE_POOL_SUB_ACCOUNT, REWARD_POOL_SUB_ACCOUNT};
use types::TokenSymbol;

use sns_rewards_api_canister::set_reserve_transfer_amounts::{
    Args as SetReserveTransferAmountsArgs, Response as SetReserveTransferAmountsResponse,
};
use sns_rewards_api_canister::set_reserve_transfer_amounts_validate::{
    Args as SetReserveTransferAmountsValidateArgs,
    Response as SetReserveTransferAmountsValidateResponse,
};

use crate::{
    client::{
        icrc1::client::{balance_of, transfer},
        rewards::{
            get_reserve_transfer_amounts, set_reserve_transfer_amounts,
            set_reserve_transfer_amounts_validate,
        },
    },
    sns_rewards_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};

fn is_set_reserve_pool_distribution_fail(value: &SetReserveTransferAmountsResponse) -> bool {
    matches!(value, SetReserveTransferAmountsResponse::InternalError(_))
}

#[test]
fn test_reserve_pool_distribution_happy_path() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();

    let goldao_ledger_id = test_env
        .token_ledgers
        .get("goldao_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let reward_pool = Account {
        owner: rewards_canister_id,
        subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
    };

    let reserve_pool_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(RESERVE_POOL_SUB_ACCOUNT),
    };

    // setup always gives a starting amount to reward pools
    let goldao_reward_pool_balance = balance_of(&pic, goldao_ledger_id, reward_pool);
    assert_eq!(goldao_reward_pool_balance, Nat::from(100_000_000_000u64));

    // set the daily reserve transfer amount
    let goldao_token = TokenSymbol::GOLDAO;
    let mut amounts = HashMap::new();
    amounts.insert(goldao_token, Nat::from(500_000_000u64));

    let res = set_reserve_transfer_amounts(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &(SetReserveTransferAmountsArgs {
            transfer_amounts: amounts,
        }),
    );
    assert_eq!(res, SetReserveTransferAmountsResponse::Success);
    tick_n_blocks(&pic, 50);

    // TRIGGER - reserve_pool_distribution cron job
    pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&pic, 100);

    // reward pool should be the same since there was nothing in the reserve pool to transfer
    let goldao_reward_pool_balance = balance_of(&pic, goldao_ledger_id, reward_pool);
    assert_eq!(goldao_reward_pool_balance, Nat::from(100_000_000_000u64));

    // transfer some goldao to the reserve pool
    transfer(
        &pic,
        test_env.sns_gov_canister_id.clone(),
        goldao_ledger_id,
        None,
        reserve_pool_account,
        (100_000_000_000u64).into(),
    )
    .unwrap();
    tick_n_blocks(&pic, 100);

    // TRIGGER - reserve_pool_distribution cron job
    pic.advance_time(Duration::from_millis(DAY_IN_MS) + Duration::from_secs(10));
    tick_n_blocks(&pic, 100);

    // reward pool should now have the same as the intial + 1 x reserve pool transfer
    let goldao_reward_pool_balance = balance_of(&pic, goldao_ledger_id, reward_pool);
    let expected_balance_reward_pool = Nat::from(100_000_000_000u64 + 500_000_000u64); // reward pool starts with 100_000_000_000 in test_env
    assert_eq!(goldao_reward_pool_balance, expected_balance_reward_pool);
}

#[test]
#[should_panic(
    expected = "FATAL ERROR: PocketIC returned a rejection error: reject code CanisterReject, reject message Caller is not a governance principal, error code CanisterRejectedMessage"
)]
fn test_set_reserve_transfer_amounts_when_caller_is_not_governance_principal() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::ICP;
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountsArgs {
        transfer_amounts: amounts,
    };

    // should fail - caller is not the governance principal
    let res = set_reserve_transfer_amounts(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &reserve_args,
    );

    assert!(is_set_reserve_pool_distribution_fail(&res));
}

#[test]
fn test_set_reserve_transfer_amounts_when_caller_is_governance_principal() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::ICP;
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountsArgs {
        transfer_amounts: amounts.clone(),
    };

    // should succeed
    let res = set_reserve_transfer_amounts(&pic, sns_gov_id, rewards_canister_id, &reserve_args);

    assert_eq!(res, SetReserveTransferAmountsResponse::Success);

    // verify the correct reserve amounts have been set
    let res = get_reserve_transfer_amounts(&pic, Principal::anonymous(), rewards_canister_id, &());
    assert_eq!(res, amounts);
}

#[test]
#[should_panic(
    expected = "FATAL ERROR: PocketIC returned a rejection error: reject code CanisterReject, reject message Caller is not a governance principal, error code CanisterRejectedMessage"
)]
fn test_set_reserve_transfer_amounts_validate_when_caller_is_not_governance_principal() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::ICP;
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountsValidateArgs {
        transfer_amounts: amounts,
    };

    // should panic
    set_reserve_transfer_amounts_validate(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &reserve_args,
    )
    .unwrap();
}

#[test]
fn test_set_reserve_transfer_amounts_validate() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::ICP;
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountsValidateArgs {
        transfer_amounts: amounts,
    };

    // should succeed
    let res =
        set_reserve_transfer_amounts_validate(&pic, sns_gov_id, rewards_canister_id, &reserve_args);
    assert!(matches!(
        res,
        SetReserveTransferAmountsValidateResponse::Ok(_)
    ))
}

#[test]
fn test_set_reserve_transfer_amounts_should_overwrite_previous_state() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::ICP;
    let ogy_token = TokenSymbol::OGY;
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountsArgs {
        transfer_amounts: amounts.clone(),
    };

    // should succeed - caller is root nns key
    let res = set_reserve_transfer_amounts(&pic, sns_gov_id, rewards_canister_id, &reserve_args);

    assert_eq!(res, SetReserveTransferAmountsResponse::Success);

    // verify the correct reserve amounts have been set
    let res = get_reserve_transfer_amounts(&pic, Principal::anonymous(), rewards_canister_id, &());
    assert_eq!(res, amounts);

    // only insert ogy
    let mut amounts = HashMap::new();
    amounts.insert(ogy_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountsArgs {
        transfer_amounts: amounts.clone(),
    };

    let res = set_reserve_transfer_amounts(&pic, sns_gov_id, rewards_canister_id, &reserve_args);

    assert_eq!(res, SetReserveTransferAmountsResponse::Success);

    // verify the correct reserve amounts have been set
    let res = get_reserve_transfer_amounts(&pic, Principal::anonymous(), rewards_canister_id, &());
    assert_eq!(res, amounts);
}
