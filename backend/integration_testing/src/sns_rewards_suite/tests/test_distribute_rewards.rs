use std::time::{ Duration, SystemTime };

use candid::{ Nat, Principal };
use canister_time::{ DAY_IN_MS, HOUR_IN_MS };
use icrc_ledger_types::icrc1::account::Account;
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::{
    get_historic_payment_round::{ self, Args as GetHistoricPaymentRoundArgs },
    payment_round::PaymentStatus,
    subaccounts::REWARD_POOL_SUB_ACCOUNT,
};
use types::TokenSymbol;

use crate::{
    client::{
        icrc1::client::{ balance_of, transfer },
        rewards::{
            force_payment_round_to_fail,
            get_active_payment_rounds,
            get_historic_payment_round,
            get_neuron_by_id,
        },
    },
    sns_rewards_suite::setup::{ default_test_setup, setup::setup_reward_pools },
    utils::{ is_interval_more_than_7_days, tick_n_blocks, HOURS_IN_WEEK },
};

#[test]
fn test_distribute_rewards_happy_path() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let ogy_token = TokenSymbol::parse("OGY").unwrap();
    let gldgov_token = TokenSymbol::parse("GLDGov").unwrap();

    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();

    // ********************************
    // 1. Distribute rewards
    // ********************************

    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 1)); //
    tick_n_blocks(&test_env.pic, 10);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6)); // 15:00
    tick_n_blocks(&test_env.pic, 20);

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
    let neuron_icp_balance = balance_of(&test_env.pic, icp_ledger_id, neuron_sub_account);
    assert_eq!(neuron_icp_balance, expected_reward);
    test_env.pic.tick();

    // ********************************
    // 3. Distribute rewards
    // ********************************

    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );

    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(3);
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 18)); // 9am
    tick_n_blocks(&test_env.pic, 30);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6 + DAY_IN_MS * 6)); // 3pm
    tick_n_blocks(&test_env.pic, 30);

    let neuron_sub_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    let neuron_icp_balance = balance_of(&test_env.pic, icp_ledger_id, neuron_sub_account);
    assert_eq!(neuron_icp_balance, expected_reward * 2);

    // ********************************
    // 4. There should be no active payment rounds
    // ********************************

    let active_payment_rounds = get_active_payment_rounds(
        &test_env.pic,
        controller,
        rewards_canister_id,
        &()
    );
    assert_eq!(active_payment_rounds.len(), 0);

    // ********************************
    // 4. neuron should have rewarded maturity
    // ********************************

    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        controller,
        rewards_canister_id,
        &neuron_id_1
    ).unwrap();
    let rewarded_mat_icp = single_neuron.rewarded_maturity.get(&icp_token).unwrap();
    let rewarded_mat_ogy = single_neuron.rewarded_maturity.get(&ogy_token).unwrap();
    let rewarded_mat_gldgov = single_neuron.rewarded_maturity.get(&gldgov_token).unwrap();
    assert_eq!(rewarded_mat_icp, &200_000u64);
    assert_eq!(rewarded_mat_ogy, &200_000u64);
    assert_eq!(rewarded_mat_gldgov, &200_000u64);
}

// if there are no rewards in the reward pool then it should not distribute for that token. other's with rewards should carry on.
#[test]
fn test_distribute_rewards_with_no_rewards() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let rewards_canister_id = test_env.rewards_canister_id;
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let ogy_token = TokenSymbol::parse("OGY").unwrap();
    let gldgov_token = TokenSymbol::parse("GLDGov").unwrap();

    let reward_pool = Account {
        owner: rewards_canister_id,
        subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
    };

    // ********************************
    // 1. Remove the entire balance of only the ICP reward pool
    // ********************************

    transfer(
        &mut test_env.pic,
        rewards_canister_id,
        icp_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - 10_000u128
    ).unwrap();

    let icp_reward_pool_balance = balance_of(&test_env.pic, icp_ledger_id, reward_pool);
    assert_eq!(icp_reward_pool_balance, Nat::from(0u64));

    // ********************************
    // 2. Distribute rewards
    // ********************************

    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 1)); //
    tick_n_blocks(&test_env.pic, 10);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6)); // 15:00
    tick_n_blocks(&test_env.pic, 20);

    // there should be no historic or active rounds for ICP because it didn't have any rewards to pay out
    let res = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args { token: icp_token.clone(), round_id: 1 })
    );
    assert_eq!(res.len(), 0);

    let res = get_active_payment_rounds(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &()
    );
    assert_eq!(res.len(), 0);

    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &neuron_id_1
    ).unwrap();
    let rewarded_mat_icp = single_neuron.rewarded_maturity.get(&icp_token.clone());
    let rewarded_mat_ogy = single_neuron.rewarded_maturity.get(&ogy_token).unwrap();
    let rewarded_mat_gldgov = single_neuron.rewarded_maturity.get(&gldgov_token).unwrap();

    assert_eq!(rewarded_mat_icp, None);
    assert_eq!(rewarded_mat_ogy, &100_000u64);
    assert_eq!(rewarded_mat_gldgov, &100_000u64);

    // ********************************
    // 3. Distribute rewards - week 3 - ALL THREE now have rewards to distribute
    // ********************************
    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );
    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(3);
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 18)); // 9am
    tick_n_blocks(&test_env.pic, 30);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6 + DAY_IN_MS * 6)); // 3pm
    tick_n_blocks(&test_env.pic, 30);

    // test historic rounds - note, payment round id's always go up by 1 if any rewards from any token are distributed so we get ("ICP".to_string(), 1)
    let res = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args { token: icp_token.clone(), round_id: 2 })
    );
    assert_eq!(res.len(), 1);

    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &neuron_id_1
    ).unwrap();
    let rewarded_mat_icp = single_neuron.rewarded_maturity.get(&icp_token).unwrap();
    let rewarded_mat_ogy = single_neuron.rewarded_maturity.get(&ogy_token).unwrap();
    let rewarded_mat_gldgov = single_neuron.rewarded_maturity.get(&gldgov_token).unwrap();
    assert_eq!(rewarded_mat_icp, &200_000u64);
    assert_eq!(rewarded_mat_ogy, &200_000u64);
    assert_eq!(rewarded_mat_gldgov, &200_000u64);
}

// if 1 reward pool doesn't have enough rewards it should be skipped
#[test]
fn test_distribute_rewards_with_not_enough_rewards() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let ogy_ledger_id = test_env.token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let ogy_token = TokenSymbol::parse("OGY").unwrap();
    let gldgov_token = TokenSymbol::parse("GLDGov").unwrap();

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
        &mut test_env.pic,
        rewards_canister_id,
        icp_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - 10_000u128 - (bad_starting_reward_amount as u128)
    ).unwrap();

    let icp_reward_pool_balance = balance_of(&test_env.pic, icp_ledger_id, reward_pool);
    assert_eq!(icp_reward_pool_balance, Nat::from(bad_starting_reward_amount));

    let ogy_reward_pool_balance = balance_of(&test_env.pic, ogy_ledger_id, reward_pool);
    assert_eq!(ogy_reward_pool_balance, Nat::from(100_000_000_000u64));

    let gldgov_reward_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reward_pool);
    assert_eq!(gldgov_reward_pool_balance, Nat::from(100_000_000_000u64));

    // ********************************
    // 2. Distribute rewards
    // ********************************

    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 1)); //
    tick_n_blocks(&test_env.pic, 10);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6)); // 15:00
    tick_n_blocks(&test_env.pic, 20);

    // there should be no historic payment round for ICP
    let res = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args { token: icp_token, round_id: 1 })
    );
    assert_eq!(res.len(), 0);
    // there should be no active round for ICP
    let p = get_active_payment_rounds(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &()
    );
    assert_eq!(p.len(), 0);

    // the others should have historic rounds
    let res = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args { token: ogy_token, round_id: 1 })
    );
    assert_eq!(res.len(), 1);
    let res = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args { token: gldgov_token, round_id: 1 })
    );
    assert_eq!(res.len(), 1);
}

#[test]
fn test_distribute_rewards_adds_to_history_correctly() {
    let mut test_env = default_test_setup();
    // test_env.pic.set_time(SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(1718776800000)); // Wednesday Jun 19, 2024, 6:00:00 AM

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let ogy_ledger_id = test_env.token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let rewards_canister_id = test_env.rewards_canister_id;

    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let ogy_token = TokenSymbol::parse("OGY").unwrap();
    let gldgov_token = TokenSymbol::parse("GLDGov").unwrap();

    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();

    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 1)); //
    tick_n_blocks(&test_env.pic, 10);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6)); // 15:00
    tick_n_blocks(&test_env.pic, 20);

    // ********************************
    // 2. Check the history
    // ********************************

    let historic_icp_rounds = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 1,
        })
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    test_env.pic.tick();

    // ********************************
    // 3. Distribute rewards
    // ********************************

    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );

    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(3);
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 18)); // 9am
    tick_n_blocks(&test_env.pic, 30);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6 + DAY_IN_MS * 6)); // 3pm
    tick_n_blocks(&test_env.pic, 30);

    // ********************************
    // 4. Check the history
    // ********************************

    let historic_icp_rounds = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 2,
        })
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    test_env.pic.tick();

    // ********************************
    // 5. Distribute rewards
    // ********************************
    // give all reward pools tokens
    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );
    tick_n_blocks(&test_env.pic, 50);
    // remove all tokens from OGY reward pool
    transfer(
        &mut test_env.pic,
        rewards_canister_id,
        ogy_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - 200_000u128
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(4);
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 18)); // 9am
    tick_n_blocks(&test_env.pic, 30);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6 + DAY_IN_MS * 6)); // 3pm
    tick_n_blocks(&test_env.pic, 30);

    // ********************************
    // 6. Check the history
    // ********************************

    let historic_icp_rounds = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 3,
        })
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    test_env.pic.tick();

    // ********************************
    // 7. Distribute rewards
    // ********************************
    // increase maturity of neurons

    // give all reward pools tokens
    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );
    // remove all tokens from OGY reward pool
    transfer(
        &mut test_env.pic,
        rewards_canister_id,
        ogy_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - 200_000u128
    ).unwrap();

    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(5);
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 18)); // 9am
    tick_n_blocks(&test_env.pic, 30);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6 + DAY_IN_MS * 6)); // 3pm
    tick_n_blocks(&test_env.pic, 30);

    // ********************************
    // 8. Check the history
    // ********************************

    let historic_icp_rounds = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: icp_token.clone(),
            round_id: 4,
        })
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    test_env.pic.tick();

    // ********************************
    // 9. Distribute rewards
    // ********************************

    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );

    // Trigger - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(6);
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 18)); // 9am
    tick_n_blocks(&test_env.pic, 30);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6 + DAY_IN_MS * 6)); // 3pm
    tick_n_blocks(&test_env.pic, 30);

    // ********************************
    // 10. Check the history
    // ********************************

    let historic_icp_rounds = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        test_env.rewards_canister_id,
        &(GetHistoricPaymentRoundArgs {
            token: ogy_token.clone(),
            round_id: 5,
        })
    );
    assert_eq!(historic_icp_rounds.len(), 1);
    test_env.pic.tick();
}

#[test]
fn test_distribution_occurs_within_correct_time_intervals() {
    let mut test_env = default_test_setup();
    let controller = test_env.controller;
    let rewards_canister_id = test_env.rewards_canister_id;
    let icp_token = TokenSymbol::parse("ICP").unwrap();
    // ********************************
    // 2. Distribute rewards - first week
    // ********************************
    tick_n_blocks(&test_env.pic, 10);
    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );
    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 1)); //
    tick_n_blocks(&test_env.pic, 10);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6)); // 15:00
    tick_n_blocks(&test_env.pic, 20);

    // ********************************
    // 2. Distribute rewards - second week
    // ********************************

    tick_n_blocks(&test_env.pic, 2);
    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );
    tick_n_blocks(&test_env.pic, 10);

    test_env.simulate_neuron_voting(3);
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 18)); // 9am
    tick_n_blocks(&test_env.pic, 30);

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6 + DAY_IN_MS * 6)); // 3pm
    tick_n_blocks(&test_env.pic, 30);

    // ********************************
    // 3. Verify more than 7 days passed between both historic payment rounds
    // ********************************

    let distribution_1_record = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args { token: icp_token.clone(), round_id: 1 })
    );
    let distribution_2_record = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args { token: icp_token.clone(), round_id: 2 })
    );
    assert_eq!(distribution_1_record.len(), 1);
    assert_eq!(distribution_2_record.len(), 1);
    let first_distribution_time = distribution_1_record[0].1.date_initialized;
    let second_distribution_time = distribution_2_record[0].1.date_initialized;
    assert!(is_interval_more_than_7_days(first_distribution_time, second_distribution_time));

    // *********************************
    // 3. Test distributions didn't occur between the 7 days
    // *********************************

    test_env.simulate_neuron_voting(4);
    tick_n_blocks(&test_env.pic, 2);
    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );
    tick_n_blocks(&test_env.pic, 10);

    for i in 0..HOURS_IN_WEEK.clone() - 2 {
        // TRIGGER - synchronize_neurons
        test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 1));
        tick_n_blocks(&test_env.pic, 1);
        // check for a distribution 1 day in
        let distribution_3_record = get_historic_payment_round(
            &test_env.pic,
            Principal::anonymous(),
            rewards_canister_id,
            &(get_historic_payment_round::Args { token: icp_token.clone(), round_id: 3 })
        );
        println!("/// i is {}", i);
        assert_eq!(distribution_3_record.len(), 0);
    }

    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 2));
    tick_n_blocks(&test_env.pic, 50);
    // check for a distribution 1 day in
    let distribution_3_record = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args { token: icp_token.clone(), round_id: 3 })
    );
    assert_eq!(distribution_3_record.len(), 1);
}

#[test]
fn test_distribution_interval_is_consistant_across_upgrades() {
    let mut test_env = default_test_setup();
    let controller = test_env.controller;
    let rewards_canister_id = test_env.rewards_canister_id;
    let icp_token = TokenSymbol::parse("ICP").unwrap();
    // ********************************
    // 2. Distribute rewards - first week
    // ********************************
    tick_n_blocks(&test_env.pic, 10);
    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );
    tick_n_blocks(&test_env.pic, 10);

    // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 1)); //
    tick_n_blocks(&test_env.pic, 10);

    // trigger the upgrade
    test_env.upgrade_rewards_canister();

    // TRIGGER - distribution
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 6)); // 15:00
    tick_n_blocks(&test_env.pic, 20);

    // ********************************
    // 3. There should be 1 historic payment round even though we upgraded
    // ********************************

    let distribution_1_record = get_historic_payment_round(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &(get_historic_payment_round::Args { token: icp_token.clone(), round_id: 1 })
    );
    assert_eq!(distribution_1_record.len(), 1);
}

#[cfg(feature = "inttest")]
#[test]
fn test_distribution_recovery() {
    let mut test_env = default_test_setup();
    let controller = test_env.controller;
    let rewards_canister_id = test_env.rewards_canister_id;
    let icp_token = TokenSymbol::parse("ICP").unwrap();
    let sns_gov_id = test_env.sns_gov_canister_id;
    let neurons: Vec<NeuronId> = test_env.neuron_data
        .iter()
        .map(|(a, n)| n.id.clone().unwrap().clone())
        .collect();
    // ********************************
    // 2. Distribute rewards - first week
    // ********************************
    test_env.simulate_neuron_voting(2);
    tick_n_blocks(&test_env.pic, 10);
    setup_reward_pools(
        &mut test_env.pic,
        &test_env.sns_gov_canister_id,
        &rewards_canister_id,
        &test_env.token_ledgers.values().cloned().collect(),
        100_000_000_000u64
    );
    // allow neuron data to sync
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS * 1));
    tick_n_blocks(&test_env.pic, 10);

    // create a new payment round for all three token types with all payments failed
    force_payment_round_to_fail(&mut test_env.pic, sns_gov_id, rewards_canister_id, &neurons);
    tick_n_blocks(&test_env.pic, 10);

    // check all the payments are failed
    let active_rounds = get_active_payment_rounds(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &()
    );
    assert_eq!(active_rounds.len(), 3);
    for round in active_rounds {
        for (_, (_, payment_status, _)) in round.payments {
            assert_eq!(payment_status, PaymentStatus::Failed(format!("Fake testing failure")));
        }
    }

    // wait 1 hour.
    test_env.pic.advance_time(Duration::from_millis(HOUR_IN_MS * 2));
    tick_n_blocks(&test_env.pic, 10);

    let active_rounds = get_active_payment_rounds(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &()
    );
    assert_eq!(active_rounds.len(), 0);
}
