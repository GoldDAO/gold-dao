use crate::client::pocket::{execute_query, execute_update};
use crate::sns_rewards_suite::setup::setup_rewards::upgrade_rewards_canister;
use crate::wasms;
use crate::{sns_rewards_suite::setup::default_test_setup, utils::tick_n_blocks};
use candid::CandidType;
use candid::Nat;
use candid::{encode_one, Principal};
use canister_time::{DAY_IN_MS, HOUR_IN_MS};
use pocket_ic::PocketIc;
use serde::Deserialize;
use serde::Serialize;
use sns_rewards_api_canister::get_historic_payment_round::Response as GetHistoricPaymentRoundsResponse;
use sns_rewards_api_canister::init::InitArgs;
use sns_rewards_api_canister::set_reserve_transfer_amounts::Response as SetReserveTransferAmountsResponse;
use sns_rewards_api_canister::set_reward_token_types::Response as SetRewartTokenTypesResponse;
use sns_rewards_api_canister::Args;
use std::collections::HashMap;
use std::time::Duration;
use types::BuildVersion;

#[test]
fn test_migration_happy_path() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let sns_rewards_id = test_env.rewards_canister_id;
    setup_old_rewards_canister(
        &pic,
        sns_rewards_id,
        &test_env.token_ledgers,
        test_env.sns_gov_canister_id,
        &test_env.controller,
    );
    let rewards_canister_id = test_env.rewards_canister_id;

    // ********************************
    // 1. Distribute rewards
    // ********************************
    let n = pic.get_time();
    println!("now is : {n:?}");
    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    tick_n_blocks(&pic, 20);
    pic.advance_time(Duration::from_millis(DAY_IN_MS)); // 9:00am Wednesday 19th June

    tick_n_blocks(&pic, 100);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 5)); // 14:00
    tick_n_blocks(&pic, 40);

    // test historic rounds - note, payment round id's always go up by 1 if any rewards from any token are distributed so we get ("ICP".to_string(), 1)
    let icp_token = TokenSymbolV0::parse("ICP").unwrap();
    let _res: GetHistoricPaymentRoundsResponse = execute_query(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        "get_historic_payment_round",
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 2,
        }),
    );

    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountsArgsV0 {
        transfer_amounts: amounts.clone(),
    };

    let res: SetReserveTransferAmountsResponse = execute_update(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        "set_reserve_transfer_amounts",
        &reserve_args,
    );

    assert_eq!(res, SetReserveTransferAmountsResponse::Success);

    let res: ReserveTokenAmountsResponse = execute_query(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        "get_reserve_transfer_amounts",
        &(),
    );
    println!("res {:?}", res);

    upgrade_rewards_canister(&pic, sns_rewards_id, &test_env.controller).unwrap();
}

#[test]
fn test_migration_unsupported_token() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let sns_rewards_id = test_env.rewards_canister_id;
    setup_old_rewards_canister(
        &pic,
        sns_rewards_id,
        &test_env.token_ledgers,
        test_env.sns_gov_canister_id,
        &test_env.controller,
    );
    let rewards_canister_id = test_env.rewards_canister_id;

    let reserve_args = SetRewardTokenTypesArgs {
        token_list: vec![(
            "UNSUPPORTED".to_string(),
            types::TokenInfo {
                ledger_id: Principal::anonymous(),
                fee: 10_000,
                decimals: 8,
            },
        )],
    };

    let _res: SetRewartTokenTypesResponse = execute_update(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        "set_reward_token_types",
        &reserve_args,
    );
    println!("res {:?}", _res);

    // ********************************
    // 1. Distribute rewards
    // ********************************
    let n = pic.get_time();
    println!("now is : {n:?}");
    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    tick_n_blocks(&pic, 20);
    pic.advance_time(Duration::from_millis(DAY_IN_MS)); // 9:00am Wednesday 19th June

    tick_n_blocks(&pic, 100);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 5)); // 14:00
    tick_n_blocks(&pic, 40);

    // test historic rounds - note, payment round id's always go up by 1 if any rewards from any token are distributed so we get ("ICP".to_string(), 1)
    let icp_token = TokenSymbolV0::parse("ICP").unwrap();
    let _res: GetHistoricPaymentRoundsResponse = execute_query(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        "get_historic_payment_round",
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 2,
        }),
    );

    let mut amounts = HashMap::new();
    amounts.insert(icp_token, Nat::from(123456789123456789u64));
    let reserve_args = SetReserveTransferAmountsArgsV0 {
        transfer_amounts: amounts.clone(),
    };

    let res: SetReserveTransferAmountsResponse = execute_update(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        "set_reserve_transfer_amounts",
        &reserve_args,
    );

    assert_eq!(res, SetReserveTransferAmountsResponse::Success);

    let res: ReserveTokenAmountsResponse = execute_query(
        &pic,
        test_env.sns_gov_canister_id,
        rewards_canister_id,
        "get_reserve_transfer_amounts",
        &(),
    );
    println!("res {:?}", res);

    let result = upgrade_rewards_canister(&pic, sns_rewards_id, &test_env.controller);
    match result {
        Ok(_) => panic!("Expected upgrade to fail due to unsupported token, but it succeeded."),
        Err(reject_response) => {
            assert!(reject_response
                .reject_message
                .contains("Failed to parse token symbol"));
        }
    }
}

pub fn setup_old_rewards_canister(
    pic: &PocketIc,
    sns_rewards_id: Principal,
    token_ledgers: &HashMap<String, Principal>,
    sns_canister_id: Principal,
    controller: &Principal,
) -> Principal {
    let rewards_wasm = wasms::REWARDS_OLD.clone();
    pic.add_cycles(sns_rewards_id, 100_000_000_000_000_000);
    pic.set_controllers(
        sns_rewards_id,
        Some(controller.clone()),
        vec![controller.clone()],
    )
    .unwrap();
    pic.tick();

    let icp_ledger_canister_id = token_ledgers
        .get("icp_ledger_canister_id")
        .expect("couldn't find ledger with 'icp_ledger_canister_id'")
        .clone();
    let sns_ledger_canister_id = token_ledgers
        .get("goldao_ledger_canister_id")
        .expect("couldn't find ledger with 'goldao_ledger_canister_id'")
        .clone();
    let ogy_ledger_canister_id = token_ledgers
        .get("ogy_ledger_canister_id")
        .expect("couldn't find ledger with 'ogy_ledger_canister_id'")
        .clone();

    let init_args = Args::Init(InitArgs {
        test_mode: true,
        version: BuildVersion::min(),
        commit_hash: "Test".to_string(),
        icp_ledger_canister_id,
        sns_ledger_canister_id,
        ogy_ledger_canister_id,
        sns_gov_canister_id: sns_canister_id.clone(),
    });
    let _ = pic
        .reinstall_canister(
            sns_rewards_id,
            rewards_wasm,
            encode_one(init_args).unwrap(),
            Some(controller.clone()),
        )
        .unwrap();
    sns_rewards_id
}

#[derive(
    Debug, Clone, CandidType, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct TokenSymbolV0(pub String);

impl TokenSymbolV0 {
    pub fn parse(symbol: &str) -> Result<TokenSymbolV0, TokenSymbolParseError> {
        const ALLOWED_TOKENS: [&str; 3] = ["ICP", "OGY", "GLDGov"];

        let valid_token = ALLOWED_TOKENS.contains(&symbol);
        if valid_token {
            Ok(TokenSymbolV0(symbol.to_string()))
        } else {
            Err(TokenSymbolParseError::InvalidTokenSymbol)
        }
    }
}

#[derive(Debug)]
pub enum TokenSymbolParseError {
    InvalidTokenSymbol,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetHistoricPaymentRoundArgs {
    pub token: TokenSymbolV0,
    pub round_id: u16,
}

pub type ReserveTokenAmountsV0 = HashMap<TokenSymbolV0, Nat>;
#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SetReserveTransferAmountsArgsV0 {
    pub transfer_amounts: ReserveTokenAmountsV0,
}

pub type ReserveTokenAmountsResponse = HashMap<TokenSymbolV0, Nat>;

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SetRewardTokenTypesArgs {
    pub token_list: Vec<(String, types::TokenInfo)>,
}
