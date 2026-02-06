use bity_ic_canister_time::{DAY_IN_MS, HOUR_IN_MS};
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use sns_rewards_api_canister::subaccounts::REWARD_POOL_SUB_ACCOUNT;
use std::time::Duration;
use types::TokenSymbol;

use crate::{
    client::icrc1::client::{balance_of, transfer},
    sns_rewards_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};

#[test]
fn test_distribute_gldt_rewards_happy_path() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let rewards_canister_id = test_env.rewards_canister_id;

    let gldt_ledger_id = TokenSymbol::GLDT.ledger_id(true);

    let neuron_id_1 = test_env
        .neuron_data
        .get(&0usize)
        .unwrap()
        .clone()
        .id
        .unwrap();

    tick_n_blocks(&pic, 10);

    // ********************************
    // 1. Advance time to have neurons synced and rewards distributed
    // ********************************

    let n = pic.get_time();
    println!("now is : {n:?}"); // Tue Jun 18 2024 09:00:02 GMT+0000
                                // // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    tick_n_blocks(&pic, 20);
    pic.advance_time(Duration::from_millis(15 * DAY_IN_MS)); // 9:00am Wednesday 3rd July 00:00
    tick_n_blocks(&pic, 100);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 3)); // 12:00
    tick_n_blocks(&pic, 40);

    // ********************************
    // 2. Calculate the expected reward
    // ********************************

    let fees =
        ((test_env.neuron_data.len() as u64) + 1) * TokenSymbol::GLDT.get_token_info(true).fee;
    println!("Total fees deducted: {}", fees);
    let payment_round_pool_amount = (100_000_000_000u64 - fees) as f64;
    let expected_reward = (payment_round_pool_amount / 10.0) as u64; // NOTE: there should be 10 neurons
                                                                     // assert_eq!(expected_reward, 9_999_989_000);

    pic.tick();

    // ********************************
    // 3. Check GLDT rewards were distributed correctly
    // ********************************

    let neuron_sub_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    let neuron_gldt_balance = balance_of(&pic, gldt_ledger_id, neuron_sub_account);
    assert_eq!(neuron_gldt_balance, expected_reward);
}

#[test]
fn test_distribute_gldt_rewards_with_no_rewards() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let rewards_canister_id = test_env.rewards_canister_id;
    let gldt_token = TokenSymbol::GLDT;
    let gldt_ledger_id = TokenSymbol::GLDT.ledger_id(true);

    // 1. Drain the GLDT reward pool completely
    transfer(
        &pic,
        rewards_canister_id,
        gldt_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        100_000_000_000u128 - gldt_token.get_token_info(true).fee as u128,
    )
    .unwrap();

    // ********************************
    // 2. Advance time to have neurons synced and rewards distributed
    // ********************************

    let n = pic.get_time();
    println!("now is : {n:?}"); // Tue Jun 18 2024 09:00:02 GMT+0000
                                // // TRIGGER - neuron vote & Maturity sync
    test_env.simulate_neuron_voting(2);
    tick_n_blocks(&pic, 20);
    pic.advance_time(Duration::from_millis(15 * DAY_IN_MS)); // 9:00am Wednesday 3rd July 00:00
    tick_n_blocks(&pic, 100);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 3)); // 12:00
    tick_n_blocks(&pic, 40);

    let neuron_id_1 = test_env.neuron_data.get(&0).unwrap().id.as_ref().unwrap();
    let neuron_sub_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    let balance = balance_of(&pic, gldt_ledger_id, neuron_sub_account);
    assert_eq!(balance, 0_u64);
}

#[test]
fn test_distribute_gldt_rewards_with_not_enough_rewards() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let rewards_canister_id = test_env.rewards_canister_id;
    let gldt_token = TokenSymbol::GLDT;
    let gldt_ledger_id = TokenSymbol::GLDT.ledger_id(true);

    // 1. Drain the GLDT reward pool completely
    transfer(
        &pic,
        rewards_canister_id,
        gldt_ledger_id,
        Some(REWARD_POOL_SUB_ACCOUNT),
        Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        99_980_000_000u128 - gldt_token.get_token_info(true).fee as u128,
    )
    .unwrap();
    // ********************************
    // 2. Advance time to have neurons synced and rewards distributed
    // ********************************

    let n = pic.get_time();
    println!("now is : {n:?}"); // Tue Jun 18 2024 09:00:02 GMT+0000
    test_env.simulate_neuron_voting(2);
    tick_n_blocks(&pic, 20);
    pic.advance_time(Duration::from_millis(15 * DAY_IN_MS)); // 9:00am Wednesday 3rd July 00:00
    tick_n_blocks(&pic, 100);

    // TRIGGER - distribution
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 3)); // 12:00
    tick_n_blocks(&pic, 40);

    let neuron_id_1 = test_env.neuron_data.get(&0).unwrap().id.as_ref().unwrap();
    let neuron_sub_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    let balance = balance_of(&pic, gldt_ledger_id, neuron_sub_account);
    assert_eq!(balance, 0_u64);
}

#[test]
fn test_gldt_distribution_occurs_only_on_first_wednesdays() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    let rewards_canister_id = test_env.rewards_canister_id;

    let gldt_ledger_id = TokenSymbol::GLDT.ledger_id(true);
    let neuron_id_1 = test_env.neuron_data.get(&0).unwrap().id.as_ref().unwrap();
    let neuron_sub_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };

    // ****************************************************************
    // 1. Test: Check the balance before first distribution
    // ****************************************************************
    let balance_before = balance_of(&pic, gldt_ledger_id, neuron_sub_account);
    assert_eq!(balance_before, 0_u64, "Should start with 0 balance");

    let n = pic.get_time();
    println!("now is : {n:?}"); // Tue Jun 18 2024 09:00:02 GMT+0000
    test_env.simulate_neuron_voting(2);
    tick_n_blocks(&pic, 20);
    pic.advance_time(Duration::from_millis(15 * DAY_IN_MS)); // 9:00am Wednesday 3rd July 00:00
    tick_n_blocks(&pic, 100);

    // ****************************************************************
    // 2. Trigger distribution on first Wednesday of July at 12:00
    // ****************************************************************
    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 3));
    tick_n_blocks(&pic, 40);
    let n = pic.get_time();
    println!("now is : {n:?}");

    let balance_after_first_wed = balance_of(&pic, gldt_ledger_id, neuron_sub_account);
    assert!(
        balance_after_first_wed > 0_u64,
        "Rewards should have distributed on first Wednesday of July"
    );

    // ****************************************************************
    // 3. Test: Distribution should NOT occur again the following Wednesday
    // ****************************************************************
    transfer(
        &pic,
        test_env.sns_gov_canister_id.clone(),
        gldt_ledger_id.clone(),
        None,
        rewards_canister_id,
        100_000_000_000u64.into(),
    )
    .unwrap();

    pic.advance_time(Duration::from_millis(7 * DAY_IN_MS)); // Next Wednesday (July 10)
    tick_n_blocks(&pic, 100);

    let balance_mid_month = balance_of(&pic, gldt_ledger_id, neuron_sub_account);
    assert_eq!(
        balance_mid_month, balance_after_first_wed,
        "Balance should NOT increase on mid-month Wednesdays"
    );

    // ****************************************************************
    // 4. Test: Distribution occurs on first Wednesday of August (with reschedule)
    // ****************************************************************

    let n = pic.get_time();
    println!("now is : {n:?}"); //Wed Jul 10 2024 12:00:02 GMT+0000
    test_env.simulate_neuron_voting(4);
    pic.advance_time(Duration::from_millis(27 * DAY_IN_MS)); // Tue Aug 06 2024 12:00:02 GMT+0000
    tick_n_blocks(&pic, 100);
    let n = pic.get_time();
    println!("now is : {n:?}");
    pic.advance_time(Duration::from_millis(21 * HOUR_IN_MS)); // Wed Aug 07 2024 09:00:02 GMT+0000
    tick_n_blocks(&pic, 10);
    let n = pic.get_time();
    println!("now is : {n:?}");
    pic.advance_time(Duration::from_millis(3 * HOUR_IN_MS)); // Wed Aug 07 2024 09:00:02 GMT+0000
    tick_n_blocks(&pic, 10);
    let n = pic.get_time();
    println!("now is : {n:?}");

    let balance_august = balance_of(&pic, gldt_ledger_id, neuron_sub_account);
    assert!(
        balance_august > balance_after_first_wed,
        "Rewards should distribute on first Wednesday of August"
    );
}
