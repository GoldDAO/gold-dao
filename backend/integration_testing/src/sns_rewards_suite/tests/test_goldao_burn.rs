use crate::client::icrc1::icrc1_total_supply;
use crate::client::rewards::{set_daily_goldao_burn_rate, set_daily_goldao_burn_rate_validate};
use crate::sns_rewards_suite::setup::test_setup_with_no_reward_pool_mint;
use crate::{
    client::icrc1::client::{balance_of, transfer},
    utils::tick_n_blocks,
};
use candid::{Nat, Principal};
use canister_time::DAY_IN_MS;
use icrc_ledger_types::icrc1::account::Account;
use sns_rewards_api_canister::set_daily_goldao_burn_rate::Response as SetDailyGOLDAOBurnRateResponse;
use sns_rewards_api_canister::set_daily_goldao_burn_rate_validate::Response as SetDailyGOLDAOBurnRateValidateResponse;
use sns_rewards_api_canister::subaccounts::RESERVE_POOL_SUB_ACCOUNT;
use std::time::Duration;

#[test]
fn test_goldao_burn_rate_happy_path() {
    let test_env = test_setup_with_no_reward_pool_mint();
    let pic = test_env.pic.borrow();

    let goldao_ledger_id = test_env
        .token_ledgers
        .get("goldao_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let reserve_pool_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(RESERVE_POOL_SUB_ACCOUNT),
    };
    let default_total_supply = Nat::from(1_000_000_000_000_000u64);
    let total_supply = icrc1_total_supply(&pic, Principal::anonymous(), goldao_ledger_id, &());
    assert_eq!(total_supply, default_total_supply);

    // Set the daily burn rate for GOLDAO
    let burn_rate = Nat::from(500_000_000u64);
    let res = set_daily_goldao_burn_rate(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &burn_rate,
    );
    assert!(matches!(res, SetDailyGOLDAOBurnRateResponse::Success));
    tick_n_blocks(&pic, 5);

    // Mint some GOLDAO to the reserve pool - note - this increases the total supply
    let amount_for_reserve_pool = 100_000_000_000u64;
    transfer(
        &pic,
        test_env.sns_gov_canister_id.clone(),
        goldao_ledger_id,
        None,
        reserve_pool_account,
        amount_for_reserve_pool.into(),
    )
    .unwrap();
    tick_n_blocks(&pic, 100);

    // TRIGGER - goldao burn cron job
    pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&pic, 100);

    // Note - burns don't require fees
    // Reserve pool should have less tokens in it - Note : We did not enable the reserve pool distribution to ensure we're only calculating what happens when a burn occurs
    let reserve_pool_balance = balance_of(&pic, goldao_ledger_id, reserve_pool_account);
    assert_eq!(
        reserve_pool_balance,
        Nat::from(100_000_000_000u64) - burn_rate.clone()
    );

    let total_supply = icrc1_total_supply(&pic, Principal::anonymous(), goldao_ledger_id, &());
    let expected_supply = default_total_supply + amount_for_reserve_pool - burn_rate;
    assert_eq!(total_supply, expected_supply)

    // total supply should be decreased
}

#[test]
fn test_goldao_burn_rate_when_reserve_pool_balance_is_zero() {
    let test_env = test_setup_with_no_reward_pool_mint();
    let pic = test_env.pic.borrow();

    let goldao_ledger_id = test_env
        .token_ledgers
        .get("goldao_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let reserve_pool_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(RESERVE_POOL_SUB_ACCOUNT),
    };

    // Set the daily burn rate for GOLDAO
    let burn_rate = Nat::from(500_000_000u64);
    let res = set_daily_goldao_burn_rate(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &burn_rate,
    );
    assert!(matches!(res, SetDailyGOLDAOBurnRateResponse::Success));
    tick_n_blocks(&pic, 5);

    // TRIGGER - goldao burn cron job - NOTE THAT WE SKIP ADDING TOKENS TO THE RESERVE POOL
    pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&pic, 100);

    // test that reserve pool is still 0
    let reserve_pool_balance = balance_of(&pic, goldao_ledger_id, reserve_pool_account);
    assert_eq!(reserve_pool_balance, Nat::from(0u64));

    // total supply should be the same as the default ( see setup_ledger.rs )
    let default_total_supply = Nat::from(1_000_000_000_000_000u64);
    let total_supply = icrc1_total_supply(&pic, Principal::anonymous(), goldao_ledger_id, &());
    assert_eq!(total_supply, default_total_supply);
}

#[test]
#[should_panic(
    expected = "FATAL ERROR: PocketIC returned a rejection error: reject code CanisterReject, reject message Caller is not a governance principal, error code CanisterRejectedMessage"
)]
fn test_set_daily_goldao_burn_rate_when_caller_is_not_governance_principal() {
    let test_env = test_setup_with_no_reward_pool_mint();
    let pic = test_env.pic.borrow();

    let rewards_canister_id = test_env.rewards_canister_id;

    // Set the daily burn rate for GOLDAO
    let burn_rate = Nat::from(500_000_000u64);
    set_daily_goldao_burn_rate(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &burn_rate,
    );
}

#[test]
#[should_panic(
    expected = "FATAL ERROR: PocketIC returned a rejection error: reject code CanisterReject, reject message Caller is not a governance principal, error code CanisterRejectedMessage"
)]
fn test_set_daily_goldao_burn_rate_validate_when_caller_is_not_governance_principal() {
    let test_env = test_setup_with_no_reward_pool_mint();
    let pic = test_env.pic.borrow();

    let rewards_canister_id = test_env.rewards_canister_id;

    // Set the daily burn rate for GOLDAO
    let burn_rate = Nat::from(500_000_000u64);
    set_daily_goldao_burn_rate_validate(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &burn_rate,
    )
    .unwrap();
}

#[test]
fn test_set_reserve_transfer_amounts_validate() {
    let test_env = test_setup_with_no_reward_pool_mint();
    let pic = test_env.pic.borrow();

    let rewards_canister_id = test_env.rewards_canister_id;

    // Set the daily burn rate for GOLDAO
    let burn_rate = Nat::from(500_000_000u64);
    let res = set_daily_goldao_burn_rate_validate(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &burn_rate,
    );
    assert!(matches!(res, SetDailyGOLDAOBurnRateValidateResponse::Ok(_)))
}

#[test]
fn test_set_reserve_transfer_amounts_validate_with_0_transfer_amount() {
    let test_env = test_setup_with_no_reward_pool_mint();
    let pic = test_env.pic.borrow();

    let rewards_canister_id = test_env.rewards_canister_id;

    // Set the daily burn rate for GOLDAO
    let burn_rate = Nat::from(0u64);
    let res = set_daily_goldao_burn_rate_validate(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &burn_rate,
    );
    assert!(matches!(
        res,
        SetDailyGOLDAOBurnRateValidateResponse::Err(_)
    ))
}
