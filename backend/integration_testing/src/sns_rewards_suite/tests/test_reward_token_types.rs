use std::collections::HashMap;

use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use types::{TokenInfo, TokenSymbol};

use crate::{
    client::rewards::{set_reward_token_types, set_reward_token_types_validate},
    sns_rewards_suite::setup::default_test_setup,
};

use sns_rewards_api_canister::set_reward_token_types::{
    Args as SetRewardTokenTypesArgs, Response as SetRewardTokenTypesResponse,
};

use sns_rewards_api_canister::set_reward_token_types_validate::{
    Args as SetRewardTokenTypesValidateArgs, Response as SetRewardTokenTypesValidateResponse,
};

fn is_fail_enum(value: &SetRewardTokenTypesResponse) -> bool {
    matches!(value, SetRewardTokenTypesResponse::InternalError(_))
}

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
#[should_panic(expected = "FATAL ERROR: Caller is not a governance principal")]
fn test_set_reward_token_types_when_not_sns_goverenance_principal() {
    let mut test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetRewardTokenTypesArgs {
        token_list: vec![(
            "ICP".to_string(),
            TokenInfo {
                ledger_id: Principal::anonymous(),
                fee: 10_000,
                decimals: 8,
            },
        )],
    };

    let res = set_reward_token_types(
        &mut test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &reserve_args,
    );

    assert!(is_fail_enum(&res));
}

#[test]
fn test_set_reward_token_types_when_caller_is_governance_principal() {
    let mut test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;
    let sns_gov_id = test_env.sns_gov_canister_id;

    let token_list = vec![(
        "ICP".to_string(),
        TokenInfo {
            ledger_id: Principal::anonymous(),
            fee: 10_000,
            decimals: 8,
        },
    )];
    let reserve_args = SetRewardTokenTypesArgs { token_list };

    let res = set_reward_token_types(
        &mut test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args,
    );

    assert_eq!(res, SetRewardTokenTypesResponse::Success);
}

#[test]
fn test_set_reward_token_types_with_bad_token_symbol() {
    let mut test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;
    let sns_gov_id = test_env.sns_gov_canister_id;

    let token_list = vec![(
        "WONT_WORK".to_string(),
        TokenInfo {
            ledger_id: Principal::anonymous(),
            fee: 10_000,
            decimals: 8,
        },
    )];
    let reserve_args = SetRewardTokenTypesArgs { token_list };

    let res = set_reward_token_types(
        &mut test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args,
    );
    assert!(is_fail_enum(&res));
}

#[test]
#[should_panic(expected = "FATAL ERROR: Caller is not a governance principal")]
fn test_set_reward_token_validate_when_not_governance_canister() {
    let mut test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;

    let token_list = vec![(
        "ICP".to_string(),
        TokenInfo {
            ledger_id: Principal::anonymous(),
            fee: 10_000,
            decimals: 8,
        },
    )];
    let reserve_args = SetRewardTokenTypesValidateArgs { token_list };

    set_reward_token_types_validate(
        &mut test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &reserve_args,
    )
    .unwrap();
}
#[test]
fn test_set_reward_token_validate() {
    let mut test_env = default_test_setup();

    let rewards_canister_id = test_env.rewards_canister_id;
    let sns_gov_id = test_env.sns_gov_canister_id;

    let token_list = vec![(
        "ICP".to_string(),
        TokenInfo {
            ledger_id: Principal::anonymous(),
            fee: 10_000,
            decimals: 8,
        },
    )];
    let reserve_args = SetRewardTokenTypesValidateArgs { token_list };

    let res = set_reward_token_types_validate(
        &mut test_env.pic,
        sns_gov_id,
        rewards_canister_id,
        &reserve_args,
    );

    assert!(matches!(res, SetRewardTokenTypesValidateResponse::Ok(_)));
}
