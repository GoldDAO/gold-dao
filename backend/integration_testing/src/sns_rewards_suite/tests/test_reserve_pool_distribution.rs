use std::{ collections::HashMap, time::Duration };

use candid::{ CandidType, Deserialize, Nat, Principal };
use canister_time::DAY_IN_MS;
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;

use sns_rewards_api_canister::subaccounts::{ RESERVE_POOL_SUB_ACCOUNT, REWARD_POOL_SUB_ACCOUNT };
use types::TokenSymbol;

use sns_rewards_api_canister::set_reserve_transfer_amounts::{
    Args as SetReserveTransferAmountsArgs,
    Response as SetReserveTransferAmountsResponse,
};
use sns_rewards_api_canister::set_reserve_transfer_amounts_validate::{
    Args as SetReserveTransferAmountsValidateArgs,
    Response as SetReserveTransferAmountsValidateResponse,
};

use crate::{
    client::{
        icrc1::client::{ balance_of, transfer },
        rewards::{
            get_reserve_transfer_amounts,
            set_reserve_transfer_amounts,
            set_reserve_transfer_amounts_validate,
        },
    },
    sns_rewards_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};

fn is_set_reserve_pool_distribution_fail(value: &SetReserveTransferAmountsResponse) -> bool {
    matches!(value, SetReserveTransferAmountsResponse::InternalError(_))
}

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_reserve_pool_distribution_happy_path() {
    let mut test_env = default_test_setup();

    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
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

    // set the daily reserve transfer amount
    let gldgov_token = TokenSymbol::parse("GLDGov").unwrap();
    let mut amounts = HashMap::new();
    amounts.insert(gldgov_token, Nat::from(500_000_000u64));

    let res = set_reserve_transfer_amounts(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        &(SetReserveTransferAmountsArgs {
            transfer_amounts: amounts,
        })
    );
    assert_eq!(res, SetReserveTransferAmountsResponse::Success);
    tick_n_blocks(&test_env.pic, 50);

    // TRIGGER - reserve_pool_distribution cron job
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // reward pool should be the same since there was nothing in the reserve pool to transfer
    let gldgov_reward_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reward_pool);
    assert_eq!(gldgov_reward_pool_balance, Nat::from(100_000_000_000u64));

    // transfer some gldgov to the reserve pool
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id.clone(),
        gldgov_ledger_id,
        None,
        reserve_pool_account,
        (100_000_000_000u64).into()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 100);

    // TRIGGER - reserve_pool_distribution cron job
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS) + Duration::from_secs(10));
    tick_n_blocks(&test_env.pic, 100);

    // reward pool should now have the same as the intial + 1 x reserve pool transfer
    let gldgov_reward_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reward_pool);
    let expected_balance_reward_pool = Nat::from(100_000_000_000u64 + 500_000_000u64); // reward pool starts with 100_000_000_000 in test_env
    assert_eq!(gldgov_reward_pool_balance, expected_balance_reward_pool);
}

#[test]
#[should_panic(expected = "FATAL ERROR: Caller is not a governance principal")]
fn test_set_reserve_transfer_amounts_when_caller_is_not_governance_principal() {
    let mut test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountsArgs {
        transfer_amounts: amounts,
    };

    // should fail - caller is not the governance principal
    let res = set_reserve_transfer_amounts(
        &mut test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &reserve_args
    );

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
    let reserve_args = SetReserveTransferAmountsArgs {
        transfer_amounts: amounts.clone(),
    };

    // should succeed
    let res = set_reserve_transfer_amounts(
        &mut test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args
    );

    assert_eq!(res, SetReserveTransferAmountsResponse::Success);

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
    let reserve_args = SetReserveTransferAmountsValidateArgs {
        transfer_amounts: amounts,
    };

    // should panic
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
    let reserve_args = SetReserveTransferAmountsValidateArgs {
        transfer_amounts: amounts,
    };

    // should succeed
    let res = set_reserve_transfer_amounts_validate(
        &test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args
    );
    assert!(matches!(res, SetReserveTransferAmountsValidateResponse::Ok(_)))
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
    let reserve_args = SetReserveTransferAmountsArgs {
        transfer_amounts: amounts.clone(),
    };

    // should succeed - caller is root nns key
    let res = set_reserve_transfer_amounts(
        &mut test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args
    );

    assert_eq!(res, SetReserveTransferAmountsResponse::Success);

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
    let reserve_args = SetReserveTransferAmountsArgs {
        transfer_amounts: amounts.clone(),
    };

    let res = set_reserve_transfer_amounts(
        &mut test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args
    );

    assert_eq!(res, SetReserveTransferAmountsResponse::Success);

    // verify the correct reserve amounts have been set
    let res = get_reserve_transfer_amounts(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &()
    );
    assert_eq!(res, amounts);
}
