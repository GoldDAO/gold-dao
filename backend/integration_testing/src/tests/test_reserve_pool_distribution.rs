use std::{ collections::HashMap, time::Duration };

use candid::{ CandidType, Deserialize, Nat, Principal };
use canister_time::DAY_IN_MS;
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use sns_rewards::{
    consts::{ RESERVE_POOL_SUB_ACCOUNT, REWARD_POOL_SUB_ACCOUNT },
    updates::set_reserve_transfer_amount::{
        SetReserveTransferAmountRequest,
        SetReserveTransferAmountResponse,
    },
};
use types::TokenSymbol;

use crate::{
    client::{
        icrc1::client::{ balance_of, transfer },
        rewards::{
            get_reserve_transfer_amounts,
            set_reserve_transfer_amounts,
            set_reserve_transfer_amounts_validate,
        },
    },
    setup::default_test_setup,
    utils::tick_n_blocks,
};

fn is_set_reserve_pool_distribution_fail(value: &SetReserveTransferAmountResponse) -> bool {
    matches!(value, SetReserveTransferAmountResponse::InternalError(_))
}

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_reserve_pool_distribution_happy_path() {
    let mut test_env = default_test_setup();

    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
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
    let gldgov_reward_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reward_pool);
    assert_eq!(gldgov_reward_pool_balance, Nat::from(100_000_000_000u64));

    // TRIGGER - reserve_pool_distribution cron job
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // reward pool should be the same since there was nothing in the reserve pool to transfer
    let gldgov_reward_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reward_pool);
    assert_eq!(gldgov_reward_pool_balance, Nat::from(100_000_000_000u64));

    // transfer some gldgov to the reserve pool
    transfer(
        &mut test_env.pic,
        controller,
        gldgov_ledger_id,
        None,
        reserve_pool_account,
        (100_000_000_000u64).into()
    ).unwrap();

    // TRIGGER - reserve_pool_distribution cron job
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // reward pool should now have double minus a fee
    let gldgov_reward_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reward_pool);
    let expected_balance_reward_pool = Nat::from(100_000_000_000u64 + 100_000_000u64);
    assert_eq!(expected_balance_reward_pool, gldgov_reward_pool_balance);
}

#[test]
#[should_panic(expected = "FATAL ERROR: Caller is not a governance principal")]
fn test_set_reserve_transfer_amounts_when_caller_is_not_governance_principal() {
    let mut test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountRequest {
        transfer_amounts: amounts,
    };

    // should fail - caller is not the governance principal
    let res = set_reserve_transfer_amounts(
        &mut test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &reserve_args
    ).unwrap();

    assert!(is_set_reserve_pool_distribution_fail(&res));
}

#[test]
fn test_set_reserve_transfer_amounts_when_caller_is_governance_principal() {
    let mut test_env = default_test_setup();
    let sns_gov_id = test_env.sns_gov_canister_id;

    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountRequest {
        transfer_amounts: amounts.clone(),
    };

    // should succeed
    let res = set_reserve_transfer_amounts(
        &mut test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args
    ).unwrap();

    assert_eq!(res, SetReserveTransferAmountResponse::Success);

    // verify the correct reserve amounts have been set
    let res = get_reserve_transfer_amounts(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &()
    );
    assert_eq!(res, amounts);
}

#[test]
#[should_panic(expected = "FATAL ERROR: Caller is not a governance principal")]
fn test_set_reserve_transfer_amounts_validate_when_caller_is_not_governance_principal() {
    let test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountRequest {
        transfer_amounts: amounts,
    };

    // should succeed
    set_reserve_transfer_amounts_validate(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &reserve_args
    ).unwrap();
}

#[test]
fn test_set_reserve_transfer_amounts_validate() {
    let test_env = default_test_setup();

    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountRequest {
        transfer_amounts: amounts,
    };

    // should succeed
    let res = set_reserve_transfer_amounts_validate(
        &test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args
    ).is_ok();

    assert_eq!(res, true);
}

#[test]
fn test_set_reserve_transfer_amounts_should_overwrite_previous_state() {
    let mut test_env = default_test_setup();

    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let ogy_token = TokenSymbol::parse("OGY").unwrap();
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountRequest {
        transfer_amounts: amounts.clone(),
    };

    // should succeed - caller is root nns key
    let res = set_reserve_transfer_amounts(
        &mut test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args
    ).unwrap();

    assert_eq!(res, SetReserveTransferAmountResponse::Success);

    // verify the correct reserve amounts have been set
    let res = get_reserve_transfer_amounts(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &()
    );
    assert_eq!(res, amounts);

    // only insert ogy
    let mut amounts = HashMap::new();
    amounts.insert(ogy_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountRequest {
        transfer_amounts: amounts.clone(),
    };

    let res = set_reserve_transfer_amounts(
        &mut test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args
    ).unwrap();

    assert_eq!(res, SetReserveTransferAmountResponse::Success);

    // verify the correct reserve amounts have been set
    let res = get_reserve_transfer_amounts(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &()
    );
    assert_eq!(res, amounts);
}
