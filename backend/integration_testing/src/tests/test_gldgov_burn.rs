use std::{ collections::HashMap, time::Duration };

use candid::{ CandidType, Deserialize, Nat, Principal };
use canister_time::DAY_IN_MS;
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;

use sns_rewards_api_canister::subaccounts::RESERVE_POOL_SUB_ACCOUNT;
use types::TokenSymbol;

use sns_rewards_api_canister::set_daily_gldgov_burn_rate::{
    Args as SetDailyGLDGovBurnRateArgs,
    Response as SetDailyGLDGovBurnRateResponse,
};
use sns_rewards_api_canister::set_daily_gldgov_burn_rate_validate::{
    Args as SetDailyGLDGovBurnRateValidateArgs,
    Response as SetDailyGLDGovBurnRateValidateResponse,
};

use crate::client::rewards::set_daily_gldgov_burn_rate;
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

fn is_set_burn_rate_fail(value: &SetDailyGLDGovBurnRateResponse) -> bool {
    matches!(value, SetDailyGLDGovBurnRateResponse::InternalError(_))
}

#[test]
fn test_gldgov_burn_rate_happy_path() {
    let mut test_env = default_test_setup();

    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let rewards_canister_id = test_env.rewards_canister_id;

    let minting_account = Account {
        owner: test_env.sns_gov_canister_id,
        subaccount: None,
    };

    let reserve_pool_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(RESERVE_POOL_SUB_ACCOUNT),
    };

    // setup always gives a starting amount to reward pools
    let gldgov_reward_pool_balance = balance_of(
        &test_env.pic,
        gldgov_ledger_id,
        reserve_pool_account
    );
    assert_eq!(gldgov_reward_pool_balance, Nat::from(100_000_000_000u64));

    let burn_rate = Nat::from(500_000_000u64);
    // Set the daily burn rate
    let res = set_daily_gldgov_burn_rate(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &burn_rate
    );
    assert!(matches!(res, SetDailyGLDGovBurnRateResponse::Success));
    tick_n_blocks(&test_env.pic, 5);

    // TRIGGER - gldgov burn cron job
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // reward pool should be the same since there was nothing in the reserve pool to transfer
    let reserve_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reserve_pool_account);
    assert_eq!(
        reserve_pool_balance,
        Nat::from(100_000_000_000u64) - (burn_rate + Nat::from(10_000u64))
    );
}

// #[test]
// #[should_panic(expected = "FATAL ERROR: Caller is not a governance principal")]
// fn test_set_reserve_transfer_amounts_when_caller_is_not_governance_principal() {
//     let mut test_env = default_test_setup();

//     let rewards_canister_id = test_env.rewards_canister_id;

//     let icp_token = TokenSymbol::parse("ICP").unwrap();
//     let mut amounts = HashMap::new();
//     amounts.insert(icp_token, Nat::from(123456789123456789u64));
//     let reserve_args = SetReserveTransferAmountsArgs {
//         transfer_amounts: amounts,
//     };

//     // should fail - caller is not the governance principal
//     let res = set_reserve_transfer_amounts(
//         &mut test_env.pic,
//         Principal::anonymous(),
//         rewards_canister_id,
//         &reserve_args
//     );

//     assert!(is_set_reserve_pool_distribution_fail(&res));
// }

// #[test]
// fn test_set_reserve_transfer_amounts_when_caller_is_governance_principal() {
//     let mut test_env = default_test_setup();
//     let sns_gov_id = test_env.sns_gov_canister_id;

//     let rewards_canister_id = test_env.rewards_canister_id;

//     let icp_token = TokenSymbol::parse("ICP").unwrap();
//     let mut amounts = HashMap::new();
//     amounts.insert(icp_token, Nat::from(123456789123456789u64));
//     let reserve_args = SetReserveTransferAmountsArgs {
//         transfer_amounts: amounts.clone(),
//     };

//     // should succeed
//     let res = set_reserve_transfer_amounts(
//         &mut test_env.pic,
//         sns_gov_id,
//         rewards_canister_id,
//         &reserve_args
//     );

//     assert_eq!(res, SetReserveTransferAmountsResponse::Success);

//     // verify the correct reserve amounts have been set
//     let res = get_reserve_transfer_amounts(
//         &test_env.pic,
//         Principal::anonymous(),
//         rewards_canister_id,
//         &()
//     );
//     assert_eq!(res, amounts);
// }

// #[test]
// #[should_panic(expected = "FATAL ERROR: Caller is not a governance principal")]
// fn test_set_reserve_transfer_amounts_validate_when_caller_is_not_governance_principal() {
//     let test_env = default_test_setup();

//     let rewards_canister_id = test_env.rewards_canister_id;

//     let icp_token = TokenSymbol::parse("ICP").unwrap();
//     let mut amounts = HashMap::new();
//     amounts.insert(icp_token, Nat::from(123456789123456789u64));
//     let reserve_args = SetReserveTransferAmountsValidateArgs {
//         transfer_amounts: amounts,
//     };

//     // should panic
//     set_reserve_transfer_amounts_validate(
//         &test_env.pic,
//         Principal::anonymous(),
//         rewards_canister_id,
//         &reserve_args
//     );
// }

// #[test]
// fn test_set_reserve_transfer_amounts_validate() {
//     let test_env = default_test_setup();

//     let sns_gov_id = test_env.sns_gov_canister_id;
//     let rewards_canister_id = test_env.rewards_canister_id;

//     let icp_token = TokenSymbol::parse("ICP").unwrap();
//     let mut amounts = HashMap::new();
//     amounts.insert(icp_token, Nat::from(123456789123456789u64));
//     let reserve_args = SetReserveTransferAmountsValidateArgs {
//         transfer_amounts: amounts,
//     };

//     // should succeed
//     let res = set_reserve_transfer_amounts_validate(
//         &test_env.pic,
//         sns_gov_id,
//         rewards_canister_id,
//         &reserve_args
//     );
//     assert!(matches!(res, SetReserveTransferAmountsValidateResponse::Success(_)))
// }

// #[test]
// fn test_set_reserve_transfer_amounts_should_overwrite_previous_state() {
//     let mut test_env = default_test_setup();

//     let sns_gov_id = test_env.sns_gov_canister_id;
//     let rewards_canister_id = test_env.rewards_canister_id;

//     let icp_token = TokenSymbol::parse("ICP").unwrap();
//     let ogy_token = TokenSymbol::parse("OGY").unwrap();
//     let mut amounts = HashMap::new();
//     amounts.insert(icp_token, Nat::from(123456789123456789u64));
//     let reserve_args = SetReserveTransferAmountsArgs {
//         transfer_amounts: amounts.clone(),
//     };

//     // should succeed - caller is root nns key
//     let res = set_reserve_transfer_amounts(
//         &mut test_env.pic,
//         sns_gov_id,
//         rewards_canister_id,
//         &reserve_args
//     );

//     assert_eq!(res, SetReserveTransferAmountsResponse::Success);

//     // verify the correct reserve amounts have been set
//     let res = get_reserve_transfer_amounts(
//         &test_env.pic,
//         Principal::anonymous(),
//         rewards_canister_id,
//         &()
//     );
//     assert_eq!(res, amounts);

//     // only insert ogy
//     let mut amounts = HashMap::new();
//     amounts.insert(ogy_token, Nat::from(123456789123456789u64));
//     let reserve_args = SetReserveTransferAmountsArgs {
//         transfer_amounts: amounts.clone(),
//     };

//     let res = set_reserve_transfer_amounts(
//         &mut test_env.pic,
//         sns_gov_id,
//         rewards_canister_id,
//         &reserve_args
//     );

//     assert_eq!(res, SetReserveTransferAmountsResponse::Success);

//     // verify the correct reserve amounts have been set
//     let res = get_reserve_transfer_amounts(
//         &test_env.pic,
//         Principal::anonymous(),
//         rewards_canister_id,
//         &()
//     );
//     assert_eq!(res, amounts);
// }
