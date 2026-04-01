use bity_ic_canister_time::{DAY_IN_MS, HOUR_IN_MS, MINUTE_IN_MS};
use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use sns_rewards_api_canister::{
    get_historic_payment_round::{self, Args as GetHistoricPaymentRoundArgs},
    subaccounts::REWARD_POOL_SUB_ACCOUNT,
};
use std::time::Duration;
use types::TokenSymbol;

use crate::{
    client::{
        icrc1::client::{balance_of, transfer},
        rewards::{get_active_payment_rounds, get_historic_payment_round, get_neuron_by_id},
    },
    sns_rewards_suite::setup::{default_test_setup, setup::setup_reward_pools},
    utils::{is_interval_more_than_7_days, tick_n_blocks},
};

#[test]
fn test_distribute_rewards_happy_path() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let controller = test_env.controller;
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::ICP;
    let ogy_token = TokenSymbol::OGY;
    let goldao_token = TokenSymbol::GOLDAO;

    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();

    tick_n_blocks(&pic, 10);

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
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 6)); // > 5 min retry window
    tick_n_blocks(&pic, 100);

    // ********************************
    // 2. Check Neuron sub account got paid correctly
    // ********************************

    let fees = (test_env.neuron_data.len() as u64) * 10_000 + 10_000;
    let payment_round_pool_amount = (100_000_000_000u64 - fees) as f64;
    let total_maturity: f64 = ((test_env.neuron_data.len() as u64) * 100_000u64) as f64;
    let percentage = (100_000 as f64) / total_maturity;
    let expected_reward = (payment_round_pool_amount * percentage) as u64;
    assert_eq!(expected_reward, 9_999_989_000);

    let neuron_sub_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    let neuron_icp_balance = balance_of(&pic, icp_ledger_id, neuron_sub_account);
    assert_eq!(neuron_icp_balance, expected_reward);
    pic.tick();

    // ********************************
    // 3. Distribute rewards
    // ********************************

    setup_reward_pools(
        &pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64,
    );

    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(3);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 19)); // 9am
    tick_n_blocks(&pic, 30);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 5 + DAY_IN_MS * 6)); // 2pm
    tick_n_blocks(&pic, 30);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 6)); // > 5 min retry window
    tick_n_blocks(&pic, 100);

    let neuron_sub_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    let neuron_icp_balance = balance_of(&pic, icp_ledger_id, neuron_sub_account);
    assert_eq!(neuron_icp_balance, expected_reward * 2);

    // ********************************
    // 4. There should be no active payment rounds
    // ********************************

    let active_payment_rounds =
        get_active_payment_rounds(&pic, controller, rewards_canister_id, &());
    assert_eq!(active_payment_rounds.len(), 0);

    // ********************************
    // 4. neuron should have rewarded maturity
    // ********************************

    let single_neuron =
        get_neuron_by_id(&pic, controller, rewards_canister_id, &neuron_id_1).unwrap();
    let rewarded_mat_icp = single_neuron.rewarded_maturity.get(&icp_token).unwrap();
    let rewarded_mat_ogy = single_neuron.rewarded_maturity.get(&ogy_token).unwrap();
    let rewarded_mat_goldao = single_neuron.rewarded_maturity.get(&goldao_token).unwrap();
    assert_eq!(rewarded_mat_icp, &200_000u64);
    assert_eq!(rewarded_mat_ogy, &200_000u64);
    assert_eq!(rewarded_mat_goldao, &200_000u64);
}

// if there are no rewards in the reward pool then it should not distribute for that token. other's with rewards should carry on.
#[test]
fn test_distribute_rewards_with_no_rewards() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;
    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let ogy_token = TokenSymbol::OGY;
    let goldao_token = TokenSymbol::GOLDAO;

    let reward_pool = Account {
        owner: rewards_canister_id,
        subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
    };

    // ********************************
    // 1. Remove the entire balance of only the ICP reward pool
    // ********************************

    transfer(
        &pic,
        rewards_canister_id,
        icp_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - 10_000u128,
    )
    .unwrap();

    let icp_reward_pool_balance = balance_of(&pic, icp_ledger_id, reward_pool);
    assert_eq!(icp_reward_pool_balance, Nat::from(0u64));

    // ********************************
    // 2. Distribute rewards
    // ********************************

    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 1)); //
    tick_n_blocks(&pic, 10);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6)); // 15:00
    tick_n_blocks(&pic, 20);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 6)); // > 5 min retry window
    tick_n_blocks(&pic, 100);

    // there should be no historic or active rounds for ICP because it didn't have any rewards to pay out
    let res = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args {
            token: icp_token.clone(),
            round_id: 1,
        }),
    );
    assert_eq!(res.len(), 0);

    let res = get_active_payment_rounds(&pic, Principal::anonymous(), rewards_canister_id, &());
    assert_eq!(res.len(), 0);

    let single_neuron = get_neuron_by_id(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &neuron_id_1,
    )
    .unwrap();
    let rewarded_mat_icp = single_neuron.rewarded_maturity.get(&icp_token.clone());
    let rewarded_mat_ogy = single_neuron.rewarded_maturity.get(&ogy_token).unwrap();
    let rewarded_mat_goldao = single_neuron.rewarded_maturity.get(&goldao_token).unwrap();

    assert_eq!(rewarded_mat_icp, None);
    assert_eq!(rewarded_mat_ogy, &100_000u64);
    assert_eq!(rewarded_mat_goldao, &100_000u64);

    // ********************************
    // 3. Distribute rewards - week 3 - ALL THREE now have rewards to distribute
    // ********************************
    setup_reward_pools(
        &pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64,
    );
    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(3);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 18)); // 9am
    tick_n_blocks(&pic, 30);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6 + DAY_IN_MS * 6)); // 3pm
    tick_n_blocks(&pic, 30);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 6)); // > 5 min retry window
    tick_n_blocks(&pic, 100);

    // test historic rounds - note, payment round id's always go up by 1 if any rewards from any token are distributed so we get ("ICP".to_string(), 1)
    let res = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args {
            token: icp_token.clone(),
            round_id: 2,
        }),
    );
    assert_eq!(res.len(), 1);

    let single_neuron = get_neuron_by_id(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &neuron_id_1,
    )
    .unwrap();
    let rewarded_mat_icp = single_neuron.rewarded_maturity.get(&icp_token).unwrap();
    let rewarded_mat_ogy = single_neuron.rewarded_maturity.get(&ogy_token).unwrap();
    let rewarded_mat_goldao = single_neuron.rewarded_maturity.get(&goldao_token).unwrap();
    assert_eq!(rewarded_mat_icp, &200_000u64);
    assert_eq!(rewarded_mat_ogy, &200_000u64);
    assert_eq!(rewarded_mat_goldao, &200_000u64);
}

// if 1 reward pool doesn't have enough rewards it should be skipped
#[test]
fn test_distribute_rewards_with_not_enough_rewards() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();

    let icp_ledger_id = test_env
        .token_ledgers
        .get("icp_ledger_canister_id")
        .unwrap()
        .clone();
    let ogy_ledger_id = test_env
        .token_ledgers
        .get("ogy_ledger_canister_id")
        .unwrap()
        .clone();
    let goldao_ledger_id = test_env
        .token_ledgers
        .get("goldao_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let ogy_token = TokenSymbol::OGY;
    let goldao_token = TokenSymbol::GOLDAO;

    // ********************************
    // 1. Give ICP reward pool balance less than the total in fees
    // ********************************
    let reward_pool = Account {
        owner: rewards_canister_id,
        subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
    };
    // calculate the minimum balance
    let minimum_reward_pool_required = 10_000u64 * (test_env.neuron_data.len() as u64) + 10_000u64;
    let bad_starting_reward_amount = minimum_reward_pool_required - 10_000;
    // transfer from reward pool to some random id
    transfer(
        &pic,
        rewards_canister_id,
        icp_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - 10_000u128 - (bad_starting_reward_amount as u128),
    )
    .unwrap();

    let icp_reward_pool_balance = balance_of(&pic, icp_ledger_id, reward_pool);
    assert_eq!(
        icp_reward_pool_balance,
        Nat::from(bad_starting_reward_amount)
    );

    let ogy_reward_pool_balance = balance_of(&pic, ogy_ledger_id, reward_pool);
    assert_eq!(ogy_reward_pool_balance, Nat::from(100_000_000_000u64));

    let goldao_reward_pool_balance = balance_of(&pic, goldao_ledger_id, reward_pool);
    assert_eq!(goldao_reward_pool_balance, Nat::from(100_000_000_000u64));

    // ********************************
    // 2. Distribute rewards
    // ********************************

    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 1)); //
    tick_n_blocks(&pic, 10);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6)); // 15:00
    tick_n_blocks(&pic, 20);

    // there should be no historic payment round for ICP
    let res = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args {
            token: icp_token,
            round_id: 1,
        }),
    );
    assert_eq!(res.len(), 0);
    // there should be no active round for ICP
    let p = get_active_payment_rounds(&pic, Principal::anonymous(), rewards_canister_id, &());
    assert_eq!(p.len(), 0);

    // the others should have historic rounds
    let res = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args {
            token: ogy_token,
            round_id: 1,
        }),
    );
    assert_eq!(res.len(), 1);
    let res = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args {
            token: goldao_token,
            round_id: 1,
        }),
    );
    assert_eq!(res.len(), 1);
}

#[test]
fn test_distribute_rewards_adds_to_history_correctly() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();

    let ogy_ledger_id = test_env
        .token_ledgers
        .get("ogy_ledger_canister_id")
        .unwrap()
        .clone();
    let rewards_canister_id = test_env.rewards_canister_id;
    tick_n_blocks(&pic, 10);
    let icp_token = TokenSymbol::ICP;
    // let ogy_token = TokenSymbol::OGY;
    let gldt_token = TokenSymbol::GLDT;
    let goldao_token = TokenSymbol::GOLDAO;

    pic.advance_time(Duration::from_millis(MINUTE_IN_MS)); //
    tick_n_blocks(&pic, 10);
    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    tick_n_blocks(&pic, 10);
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 1 + MINUTE_IN_MS)); //
    tick_n_blocks(&pic, 10);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 5)); // 14:00
    tick_n_blocks(&pic, 20);

    // ********************************
    // 2. Check the history
    // ********************************

    let historic_icp_rounds = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 1,
        }),
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    pic.tick();

    // ********************************
    // 3. Distribute rewards
    // ********************************

    setup_reward_pools(
        &pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64,
    );
    tick_n_blocks(&pic, 10);

    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(3);
    tick_n_blocks(&pic, 10);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 19)); // 9am
    tick_n_blocks(&pic, 30);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6 + DAY_IN_MS * 6)); // 3pm
    tick_n_blocks(&pic, 30);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 6)); // > 5 min retry window
    tick_n_blocks(&pic, 100);

    // ********************************
    // 4. Check the history
    // ********************************

    let historic_icp_rounds = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 2,
        }),
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    pic.tick();

    // ********************************
    // 5. Distribute rewards
    // ********************************
    // give all reward pools tokens
    setup_reward_pools(
        &pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64,
    );
    tick_n_blocks(&pic, 50);
    // remove all tokens from OGY reward pool
    transfer(
        &pic,
        rewards_canister_id,
        ogy_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - 200_000u128,
    )
    .unwrap();
    tick_n_blocks(&pic, 10);
    // let n = test_env.pic.get_time();
    // println!("now is : {n:?}"); // 15
    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(4);
    tick_n_blocks(&pic, 30);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 18)); // 9am
    tick_n_blocks(&pic, 10);
    // let n = test_env.pic.get_time();
    // println!("now is : {n:?}"); // 9:02

    pic.advance_time(Duration::from_millis(DAY_IN_MS - (2 * MINUTE_IN_MS))); // 9am
    tick_n_blocks(&pic, 30);
    // let n = test_env.pic.get_time();
    // println!("now is : {n:?}"); // 9:00

    pic.advance_time(Duration::from_millis(DAY_IN_MS + HOUR_IN_MS)); // 9am
    tick_n_blocks(&pic, 30);
    let n = pic.get_time();
    println!("now is : {n:?}"); // next day at 10:00

    // TRIGGER - GLDT distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 3 + DAY_IN_MS * 5)); // 12am
    tick_n_blocks(&pic, 30);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 6)); // > 5 min retry window
    tick_n_blocks(&pic, 100);
    // TRIGGER - ICP distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 2)); // 3pm
    tick_n_blocks(&pic, 30);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 6)); // > 5 min retry window
    tick_n_blocks(&pic, 100);

    let n = pic.get_time();
    println!("now is : {n:?}"); // next day at 10:00

    // ********************************
    // 6. Check the history
    // ********************************

    // NOTE: one round is GLDT distribution
    let historic_icp_rounds = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: gldt_token.clone(),
            round_id: 3,
        }),
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    pic.tick();

    // NOTE: the second distribution is for other tokens
    let historic_icp_rounds = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 4,
        }),
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    pic.tick();

    // ********************************
    // 7. Distribute rewards
    // ********************************
    // increase maturity of neurons

    // give all reward pools tokens
    setup_reward_pools(
        &pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64,
    );
    // remove all tokens from OGY reward pool
    transfer(
        &pic,
        rewards_canister_id,
        ogy_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - 200_000u128,
    )
    .unwrap();

    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(5);
    tick_n_blocks(&pic, 20);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 17)); // 9am
    tick_n_blocks(&pic, 20);

    pic.advance_time(Duration::from_millis(HOUR_IN_MS)); // 9am
    tick_n_blocks(&pic, 30);

    pic.advance_time(Duration::from_millis(DAY_IN_MS + (2 * MINUTE_IN_MS))); // 9am
    tick_n_blocks(&pic, 30);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS)); // 9am
    tick_n_blocks(&pic, 30);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 5 + DAY_IN_MS * 5)); // 3pm
    tick_n_blocks(&pic, 30);
    let n = pic.get_time();
    println!("now is : {n:?}");

    // ********************************
    // 8. Check the history
    // ********************************

    let historic_icp_rounds = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 4,
        }),
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    pic.tick();

    // ********************************
    // 9. Distribute rewards
    // ********************************

    setup_reward_pools(
        &pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64,
    );

    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(6);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 18)); // 9am
    tick_n_blocks(&pic, 30);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6 + DAY_IN_MS * 6)); // 3pm
    tick_n_blocks(&pic, 30);

    // ********************************
    // 10. Check the history
    // ********************************

    let historic_icp_rounds = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 5,
        }),
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    pic.tick();

    let historic_goldao_rounds = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: goldao_token.clone(),
            round_id: 5,
        }),
    );
    assert_eq!(historic_goldao_rounds.len(), 1);
    pic.tick();
}

#[test]
fn test_distribution_occurs_within_correct_time_intervals() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let rewards_canister_id = test_env.rewards_canister_id;
    let icp_token = TokenSymbol::ICP;
    let gldt_token = TokenSymbol::GLDT;
    // ********************************
    // 2. Distribute rewards - first week
    // ********************************
    tick_n_blocks(&pic, 10);
    setup_reward_pools(
        &pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64,
    );
    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 1)); //
    tick_n_blocks(&pic, 10);
    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 5)); // 15:00
    tick_n_blocks(&pic, 20);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 6)); // > 5 min retry window
    tick_n_blocks(&pic, 100);
    println!("First distribution done at {:?}", pic.get_time());

    // ********************************
    // 2. Distribute rewards - second week
    // ********************************

    tick_n_blocks(&pic, 2);
    setup_reward_pools(
        &pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64,
    );
    tick_n_blocks(&pic, 10);

    test_env.simulate_neuron_voting(3);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 19)); // 9am
    tick_n_blocks(&pic, 30);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 5 + DAY_IN_MS * 6)); // 2pm
    tick_n_blocks(&pic, 30);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 6)); // > 5 min retry window
    tick_n_blocks(&pic, 100);
    println!("Second distribution done at {:?}", pic.get_time());

    // ********************************
    // 3. Verify more than 7 days passed between both historic payment rounds
    // ********************************

    let distribution_1_record = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args {
            token: icp_token.clone(),
            round_id: 1,
        }),
    );

    let distribution_2_record = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args {
            token: icp_token.clone(),
            round_id: 2,
        }),
    );
    assert_eq!(distribution_1_record.len(), 1);
    assert_eq!(distribution_2_record.len(), 1);
    let first_distribution_time = distribution_1_record[0].1.date_initialized;
    let second_distribution_time = distribution_2_record[0].1.date_initialized;
    assert!(is_interval_more_than_7_days(
        first_distribution_time,
        second_distribution_time
    ));

    // *********************************
    // 4. Test distributions didn't occur between the 7 days
    // *********************************

    setup_reward_pools(
        &pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64,
    );
    tick_n_blocks(&pic, 10);

    test_env.simulate_neuron_voting(4);
    tick_n_blocks(&pic, 10);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 19));
    pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&pic, 10);

    pic.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 3));
    tick_n_blocks(&pic, 30);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 6)); // > 5 min retry window
    tick_n_blocks(&pic, 100);
    println!("Third distribution done at {:?}", pic.get_time());

    // NOTE: GLDT distribution would have occurred here
    let distribution_3_record = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args {
            token: gldt_token.clone(),
            round_id: 3,
        }),
    );
    assert_eq!(distribution_3_record.len(), 1);
}

#[test]
fn test_distribution_interval_is_consistant_across_upgrades() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let rewards_canister_id = test_env.rewards_canister_id;
    let icp_token = TokenSymbol::parse("ICP").unwrap();
    // ********************************
    // 2. Distribute rewards - first week
    // ********************************
    tick_n_blocks(&pic, 10);
    setup_reward_pools(
        &pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64,
    );
    tick_n_blocks(&pic, 10);

    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 1)); //
    tick_n_blocks(&pic, 10);

    // trigger the upgrade
    test_env.upgrade_rewards_canister();

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6)); // 15:00
    tick_n_blocks(&pic, 20);

    // ********************************
    // 3. There should be 1 historic payment round even though we upgraded
    // ********************************

    let distribution_1_record = get_historic_payment_round(
        &pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args {
            token: icp_token.clone(),
            round_id: 1,
        }),
    );
    assert_eq!(distribution_1_record.len(), 1);
}

// use crate::client::rewards::force_payment_round_to_fail;
// use sns_governance_canister::types::NeuronId;
// use sns_rewards_api_canister::payment_round::PaymentStatus;
// #[test]
// fn test_distribution_recovery() {
//     let test_env = default_test_setup();
//     let pic = test_env.pic.borrow();
//     let rewards_canister_id = test_env.rewards_canister_id;
//     let sns_gov_id = test_env.sns_gov_canister_id;
//     let neurons: Vec<NeuronId> = test_env
//         .neuron_data
//         .iter()
//         .map(|(_a, n)| n.id.clone().unwrap().clone())
//         .collect();
//     // ********************************
//     // 2. Distribute rewards - first week
//     // ********************************
//     test_env.simulate_neuron_voting(2);
//     tick_n_blocks(&pic, 10);
//     setup_reward_pools(
//         &pic,
//         &test_env.sns_gov_canister_id,
//         &rewards_canister_id,
//         &test_env.token_ledgers.values().cloned().collect(),
//         100_000_000_000u64,
//     );
//     // allow neuron data to sync
//     pic.advance_time(Duration::from_millis(DAY_IN_MS * 1));
//     tick_n_blocks(&pic, 10);

//     // create a new payment round for all three token types with all payments failed
//     force_payment_round_to_fail(&pic, sns_gov_id, rewards_canister_id, &neurons);
//     tick_n_blocks(&pic, 10);

//     // check all the payments are failed
//     let active_rounds =
//         get_active_payment_rounds(&pic, Principal::anonymous(), rewards_canister_id, &());
//     assert_eq!(active_rounds.len(), 3);
//     for round in active_rounds {
//         for (_, (_, payment_status, _)) in round.payments {
//             assert_eq!(
//                 payment_status,
//                 PaymentStatus::Failed(format!("Fake testing failure"))
//             );
//         }
//     }

//     // wait 1 hour.
//     pic.advance_time(Duration::from_millis(HOUR_IN_MS * 2));
//     tick_n_blocks(&pic, 10);

//     let active_rounds =
//         get_active_payment_rounds(&pic, Principal::anonymous(), rewards_canister_id, &());
//     assert_eq!(active_rounds.len(), 0);
// }
