use std::time::Duration;
use candid::{ Nat, Principal };
use canister_time::DAY_IN_MS;
use icrc_ledger_types::icrc1::account::Account;

use sns_rewards_api_canister::subaccounts::RESERVE_POOL_SUB_ACCOUNT;
use sns_rewards_api_canister::set_daily_gldgov_burn_rate::Response as SetDailyGLDGovBurnRateResponse;
use sns_rewards_api_canister::set_daily_gldgov_burn_rate_validate::Response as SetDailyGLDGovBurnRateValidateResponse;

use crate::client::rewards::{ set_daily_gldgov_burn_rate, set_daily_gldgov_burn_rate_validate };
use crate::{
    client::icrc1::client::{ balance_of, transfer },
    setup::default_test_setup,
    utils::tick_n_blocks,
};

#[test]
fn test_gldgov_burn_rate_happy_path() {
    let mut test_env = default_test_setup();

    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let rewards_canister_id = test_env.rewards_canister_id;

    let reserve_pool_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(RESERVE_POOL_SUB_ACCOUNT),
    };

    // Set the daily burn rate for GLDGov
    let burn_rate = Nat::from(500_000_000u64);
    let res = set_daily_gldgov_burn_rate(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &burn_rate
    );
    assert!(matches!(res, SetDailyGLDGovBurnRateResponse::Success));
    tick_n_blocks(&test_env.pic, 5);

    // Transfer some GLDGov to the reserve pool
    transfer(
        &mut test_env.pic,
        controller,
        gldgov_ledger_id,
        None,
        reserve_pool_account,
        (100_000_000_000u64).into()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 100);

    // TRIGGER - gldgov burn cron job
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // Reserve pool should have less tokens in it - Note : We did not enable the reserve pool distribution to ensure we're only calculating what happens when a burn occurs
    let reserve_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reserve_pool_account);
    assert_eq!(
        reserve_pool_balance,
        Nat::from(100_000_000_000u64) - (burn_rate + Nat::from(10_000u64))
    );
}

#[test]
fn test_gldgov_burn_rate_when_reserve_pool_balance_is_zero() {
    let mut test_env = default_test_setup();

    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let reserve_pool_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(RESERVE_POOL_SUB_ACCOUNT),
    };

    // Set the daily burn rate for GLDGov
    let burn_rate = Nat::from(500_000_000u64);
    let res = set_daily_gldgov_burn_rate(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &burn_rate
    );
    assert!(matches!(res, SetDailyGLDGovBurnRateResponse::Success));
    tick_n_blocks(&test_env.pic, 5);

    // TRIGGER - gldgov burn cron job - NOTE THAT WE SKIP ADDING TOKENS TO THE RESERVE POOL
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // test that reserve pool is still 0
    let reserve_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reserve_pool_account);
    assert_eq!(reserve_pool_balance, Nat::from(0u64));
}

#[test]
#[should_panic(expected = "FATAL ERROR: Caller is not a governance principal")]
fn test_set_daily_gldgov_burn_rate_when_caller_is_not_governance_principal() {
    let mut test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;

    // Set the daily burn rate for GLDGov
    let burn_rate = Nat::from(500_000_000u64);
    set_daily_gldgov_burn_rate(
        &mut test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &burn_rate
    );
}

#[test]
#[should_panic(expected = "FATAL ERROR: Caller is not a governance principal")]
fn test_set_daily_gldgov_burn_rate_validate_when_caller_is_not_governance_principal() {
    let test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;

    // Set the daily burn rate for GLDGov
    let burn_rate = Nat::from(500_000_000u64);
    set_daily_gldgov_burn_rate_validate(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &burn_rate
    );
}

#[test]
fn test_set_reserve_transfer_amounts_validate() {
    let test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;

    // Set the daily burn rate for GLDGov
    let burn_rate = Nat::from(500_000_000u64);
    let res = set_daily_gldgov_burn_rate_validate(
        &test_env.pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &burn_rate
    );
    assert!(matches!(res, SetDailyGLDGovBurnRateValidateResponse::Success(_)))
}

#[test]
fn test_set_reserve_transfer_amounts_validate_with_0_transfer_amount() {
    let test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;

    // Set the daily burn rate for GLDGov
    let burn_rate = Nat::from(0u64);
    let res = set_daily_gldgov_burn_rate_validate(
        &test_env.pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &burn_rate
    );
    assert!(matches!(res, SetDailyGLDGovBurnRateValidateResponse::Error(_)))
}
