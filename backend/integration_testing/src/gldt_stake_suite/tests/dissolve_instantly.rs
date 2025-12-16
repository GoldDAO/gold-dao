use crate::client::gldt_stake::get_position;
use crate::client::gldt_stake::get_total_staked;
use crate::client::gldt_stake::manage_stake_position_with_tick;
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::{add_rewards_to_neurons, create_stake_position_util_mock};
use crate::utils::wait_1_day;
use crate::{
    client::icrc1::client::balance_of, gldt_stake_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};
use assert_matches::assert_matches;
use bity_ic_canister_time::WEEK_IN_MS;
use bity_ic_canister_time::{DAY_IN_MS, HOUR_IN_MS};
use candid::Nat;
use candid::Principal;
use gldt_stake_api_canister::manage_stake_position;
use gldt_stake_common::accounts::INSTANT_DISSOLVEMENT_FEE_ACCOUNT;
use gldt_stake_common::ledgers::GLDT_TX_FEE;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;
use types::TokenSymbol;

#[test]
fn test_dissolve_instantly_full() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        gld_rewards_canister_id,
        neuron_data,
        ledger_fees,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers.get("gldt_ledger_canister_id").unwrap();

    // --- Create stake position ---
    let (user, _) = create_stake_position_util_mock(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        100_000_000_000u128,
    );

    let user_gldt_balance = balance_of(
        pic,
        gldt_ledger_id.clone(),
        Account {
            owner: user,
            subaccount: None,
        },
    );
    println!("User GLDT balance before withdraw: {:?}", user_gldt_balance);

    let total_staked_before_withdraw = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(total_staked_before_withdraw, Nat::from(100_000_000_000u128));

    // --- Add rewards ---
    add_rewards_to_neurons(
        pic,
        neuron_data.clone(),
        controller,
        &token_ledgers,
        gld_rewards_canister_id,
        gldt_stake_canister_id,
        ledger_fees.clone(),
    );

    wait_1_day(pic);
    wait_1_day(pic);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS));
    tick_n_blocks(pic, 10);

    // --- Check that the rewards are available ---
    let user_position = get_position(pic, user, gldt_stake_canister_id, &user).unwrap();
    let rewards = &user_position.claimable_rewards;
    assert_ne!(rewards[&TokenSymbol::GOLDAO], Nat::from(0_u64));
    assert_ne!(rewards[&TokenSymbol::OGY], Nat::from(0_u64));
    assert_ne!(rewards[&TokenSymbol::ICP], Nat::from(0_u64));

    let position_stake_amount = user_position.staked.clone();
    println!("Position stake amount: {:?}", position_stake_amount);
    let position_instant_dissolve_fee = Nat::from(0_u64);
    println!(
        "Position instant dissolve fee: {:?}",
        position_instant_dissolve_fee
    );

    wait_1_day(pic);

    // --- Claim all rewards ---
    let _res = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::ClaimRewards {
            tokens: vec![TokenSymbol::GOLDAO, TokenSymbol::OGY, TokenSymbol::ICP],
        },
    );

    let res = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::DissolveInstantly { fraction: 100 },
    );
    tick_n_blocks(pic, 10);
    println!("res: {:?}", res);

    assert_matches!(res, Ok(_));
    let position = res.unwrap();
    assert_eq!(position.staked, Nat::from(0_u64));
    tick_n_blocks(pic, 2);

    let total_staked_after_withdraw = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(total_staked_after_withdraw, Nat::from(0_u64));

    let user_gldt_balance_after_withdraw = balance_of(
        pic,
        gldt_ledger_id.clone(),
        Account {
            owner: user,
            subaccount: None,
        },
    );

    println!(
        "User GLDT balance after withdraw: {:?}",
        user_gldt_balance_after_withdraw
    );
    println!("user_gldt_balance: {:?}", user_gldt_balance);
    println!("position_stake_amount: {:?}", position_stake_amount.clone());
    println!(
        "position_instant_dissolve_fee: {:?}",
        position_instant_dissolve_fee.clone()
    );
    assert_eq!(
        user_gldt_balance_after_withdraw,
        (user_gldt_balance + position_stake_amount)
            - (position_instant_dissolve_fee.clone() + GLDT_TX_FEE)
    );

    // --- Check the position is not yet removed ---
    let user_position = get_position(pic, user, gldt_stake_canister_id, &user);
    assert!(user_position.is_some());

    // --- Check the instant dissolvement fee account ---
    pic.advance_time(Duration::from_millis(WEEK_IN_MS));
    tick_n_blocks(pic, 2);

    let fee_account_balance = balance_of(
        pic,
        gldt_ledger_id.clone(),
        Account {
            owner: gldt_stake_canister_id,
            subaccount: Some(INSTANT_DISSOLVEMENT_FEE_ACCOUNT),
        },
    );

    assert_eq!(fee_account_balance, Nat::from(0_u64));

    // Check that the user position is deleted
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 8));
    tick_n_blocks(pic, 10);
    let user_position = get_position(pic, user, gldt_stake_canister_id, &user);
    assert!(user_position.is_none());
}

#[test]
fn test_dissolve_instantly_partial() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        gld_rewards_canister_id,
        neuron_data,
        ledger_fees,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers.get("gldt_ledger_canister_id").unwrap();

    // --- Create stake position ---
    let (user, _) = create_stake_position_util_mock(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        100_000_000_000_u128,
    );

    let user_gldt_balance = balance_of(
        pic,
        gldt_ledger_id.clone(),
        Account {
            owner: user,
            subaccount: None,
        },
    );

    let total_staked_before_withdraw = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(
        total_staked_before_withdraw,
        Nat::from(100_000_000_000_u128)
    );

    add_rewards_to_neurons(
        pic,
        neuron_data.clone(),
        controller,
        &token_ledgers,
        gld_rewards_canister_id,
        gldt_stake_canister_id,
        ledger_fees.clone(),
    );

    wait_1_day(pic);
    wait_1_day(pic);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS));
    tick_n_blocks(pic, 10);

    // --- Check that the rewards are available ---
    let user_position = get_position(pic, user, gldt_stake_canister_id, &user).unwrap();
    let rewards = &user_position.claimable_rewards;
    assert_ne!(rewards[&TokenSymbol::GOLDAO], Nat::from(0_u64));
    assert_ne!(rewards[&TokenSymbol::OGY], Nat::from(0_u64));
    assert_ne!(rewards[&TokenSymbol::ICP], Nat::from(0_u64));

    let position_stake_amount = user_position.staked.clone();

    pic.advance_time(Duration::from_millis(DAY_IN_MS));

    // --- Claim all rewards ---
    let _res = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::ClaimRewards {
            tokens: vec![TokenSymbol::GOLDAO, TokenSymbol::OGY, TokenSymbol::ICP],
        },
    );

    let res = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::DissolveInstantly { fraction: 50 },
    );
    tick_n_blocks(pic, 10);
    assert_matches!(res, Ok(_));
    let position = res.unwrap();
    assert_eq!(position.staked, Nat::from(50_000_000_000_u128));
    tick_n_blocks(pic, 2);

    let total_staked_after_withdraw = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(total_staked_after_withdraw, Nat::from(50_000_000_000_u128));

    let user_gldt_balance_after_withdraw = balance_of(
        pic,
        gldt_ledger_id.clone(),
        Account {
            owner: user,
            subaccount: None,
        },
    );

    assert_eq!(
        user_gldt_balance_after_withdraw,
        (user_gldt_balance + position_stake_amount / Nat::from(2_u64))
            - (Nat::from(0_u64) + GLDT_TX_FEE)
    );

    // --- Check the instant dissolvement fee account ---
    pic.advance_time(Duration::from_millis(WEEK_IN_MS));
    tick_n_blocks(pic, 2);

    let fee_account_balance = balance_of(
        pic,
        gldt_ledger_id.clone(),
        Account {
            owner: gldt_stake_canister_id,
            subaccount: Some(INSTANT_DISSOLVEMENT_FEE_ACCOUNT),
        },
    );

    assert_eq!(fee_account_balance, Nat::from(0_u64));

    tick_n_blocks(pic, 10);

    let user_position = get_position(pic, user, gldt_stake_canister_id, &user);
    assert!(user_position.is_some());
}

#[test]
fn test_dissolve_instantly_zero_fraction_should_fail() {
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();

    let (user, _) = create_stake_position_util_mock(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        100_000_000_000u128,
    );

    let res = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::DissolveInstantly { fraction: 0 },
    );

    assert_matches!(
        res,
        Err(ManageStakePositionError::GeneralError(
            GeneralError::InvalidPercentage(_)
        ))
    );
}

#[test]
fn test_dissolve_instantly_over_100_should_fail() {
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();

    let (user, _) = create_stake_position_util_mock(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        100_000_000_000u128,
    );

    let res = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::DissolveInstantly { fraction: 150 },
    );

    assert_matches!(
        res,
        Err(ManageStakePositionError::GeneralError(
            GeneralError::InvalidPercentage(_)
        ))
    );
}

#[test]
fn test_dissolve_instantly_99_percent_leaves_1_percent() {
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();

    let (user, _) = create_stake_position_util_mock(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        100_000_000_000u128,
    );

    let res = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::DissolveInstantly { fraction: 99 },
    );
    assert_matches!(res, Ok(_));
    let position = res.unwrap();

    let expected_remaining = Nat::from(1_000_000_000u128); // 1% of 100B
    assert_eq!(position.staked, expected_remaining);
}

#[test]
fn test_dissolve_instantly_when_already_dissolved_should_fail() {
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();

    let (user, _) = create_stake_position_util_mock(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        100_000_000_000u128,
    );

    // Dissolve fully
    let _ = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::DissolveInstantly { fraction: 100 },
    );

    // Try to dissolve again
    let res = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::DissolveInstantly { fraction: 100 },
    );

    assert_matches!(
        res,
        Err(ManageStakePositionError::GeneralError(
            GeneralError::TransferError(_)
        ))
    );
}

#[test]
fn test_dissolve_instantly_as_anonymous_should_fail() {
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        gldt_stake_canister_id,
        controller,
        ..
    } = test_env;
    let pic = &pic.borrow();

    let res = manage_stake_position_with_tick(
        pic,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &manage_stake_position::Args::DissolveInstantly { fraction: 100 },
    );

    assert_matches!(
        res,
        Err(ManageStakePositionError::GeneralError(
            GeneralError::InvalidPrincipal(_)
        ))
    );
}
