use crate::client::gldt_stake::get_total_staked;
use crate::client::gldt_stake::manage_stake_position_with_tick;
use crate::gldt_stake_suite::utils::add_stake;
use crate::gldt_stake_suite::utils::create_user_with_funds;
use crate::{
    client::{gldt_stake::get_position, icrc1_icrc2_token::icrc2_approve},
    gldt_stake_suite::setup::{default_test_setup, setup::GldtStakeTestEnv},
    utils::tick_n_blocks,
};
use candid::{Encode, Nat, Principal};
use gldt_stake_api_canister::manage_stake_position;
use gldt_stake_common::ledgers::GLDT_TX_FEE;
use gldt_stake_common::manage_stake_position_interface::AddStakePositionErrors;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;

#[test]
fn add_stake_new_position_works() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers
        .get("gldt_ledger_canister_id")
        .expect("Missing GLDT ledger canister ID");

    // --- Create user and fund it ---
    let user = create_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        2_000_000_000u128,
    );

    // --- Approve stake amount for spender (gldt_stake_canister) ---
    let stake_amount = 1_000_000_000u128;
    let total_approval = stake_amount + GLDT_TX_FEE as u128;

    let approval_result = icrc2_approve(
        pic,
        user,
        gldt_ledger_id.clone(),
        &icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: gldt_stake_canister_id,
                subaccount: None,
            },
            amount: Nat::from(total_approval),
            expected_allowance: Some(Nat::from(0_u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        },
    );

    assert!(matches!(approval_result, icrc2_approve::Response::Ok(_)));
    tick_n_blocks(pic, 2);

    // --- Perform stake operation ---
    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id.clone(),
        &manage_stake_position::Args::AddStake {
            amount: Nat::from(total_approval),
        },
    );

    matches!(
        response,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );

    // --- Verify that stake position was created ---
    let position = get_position(pic, user, gldt_stake_canister_id, &user);
    assert!(position.is_none(), "Stake position should not exist");
}

#[test]
fn increase_stake_position_invalid_caller() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        gldt_stake_canister_id,
        controller,
        ..
    } = test_env;
    let pic = &pic.borrow();

    // --- Stake with anonymous caller ---
    let amount_to_add = 10_000_000_000;
    let res = manage_stake_position_with_tick(
        pic,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &manage_stake_position::Args::AddStake {
            amount: Nat::from(amount_to_add + GLDT_TX_FEE as u128),
        },
    );

    matches!(
        res,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );
}

#[test]
fn increase_stake_position_position_guard() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        gldt_stake_canister_id,
        controller,
        token_ledgers,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers
        .get("gldt_ledger_canister_id")
        .expect("Missing GLDT ledger canister ID");

    let user = create_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        4_000_000_000_u128,
    );

    // --- Add stake concurrently ---
    let amount_to_add = 10_000_000_000;
    let call1 = pic
        .submit_call(
            gldt_stake_canister_id,
            user,
            "manage_stake_position",
            Encode!(&manage_stake_position::Args::AddStake {
                amount: Nat::from(amount_to_add + GLDT_TX_FEE as u128),
            })
            .unwrap(),
        )
        .unwrap();

    let call2 = pic
        .submit_call(
            gldt_stake_canister_id,
            user,
            "manage_stake_position",
            Encode!(&manage_stake_position::Args::AddStake {
                amount: Nat::from(amount_to_add + GLDT_TX_FEE as u128),
            })
            .unwrap(),
        )
        .unwrap();

    // --- Wait for both calls to complete and check results ---
    let result1: manage_stake_position::Response =
        crate::client::pocket::unwrap_response(pic.await_call(call1));
    let result2: manage_stake_position::Response =
        crate::client::pocket::unwrap_response(pic.await_call(call2));

    matches!(
        result1,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );
    matches!(
        result2,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );
}

#[test]
fn add_stake_too_low_amount() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        gldt_stake_canister_id,
        controller,
        token_ledgers,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers
        .get("gldt_ledger_canister_id")
        .expect("Missing GLDT ledger canister ID");

    let amount_to_add = 1_000_u128;

    // --- Create user and fund it ---
    let user = create_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        amount_to_add + GLDT_TX_FEE as u128,
    );

    // --- Perform stake operation ---
    let response = add_stake(
        pic,
        user,
        gldt_ledger_id,
        gldt_stake_canister_id,
        amount_to_add,
    );

    matches!(
        response,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );
}

#[test]
fn add_stake_too_high_amount() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        gldt_stake_canister_id,
        controller,
        token_ledgers,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers
        .get("gldt_ledger_canister_id")
        .expect("Missing GLDT ledger canister ID");

    let amount_to_add = 10_000_000_000_000_000_u128;

    // --- Create user and fund it ---
    let user = create_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        amount_to_add,
    );

    // --- Perform stake operation ---
    let response = add_stake(
        pic,
        user,
        gldt_ledger_id,
        gldt_stake_canister_id,
        amount_to_add,
    );

    matches!(
        response,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );
}

#[test]
fn add_stake_new_position_without_allowance() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers
        .get("gldt_ledger_canister_id")
        .expect("Missing GLDT ledger canister ID");

    // --- Create user and fund it ---
    let user = create_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        2_000_000_000u128,
    );

    // --- Perform stake operation ---
    let stake_amount = 1_000_000_000u128;
    let total_approval = stake_amount + GLDT_TX_FEE as u128;
    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id.clone(),
        &manage_stake_position::Args::AddStake {
            amount: Nat::from(total_approval),
        },
    );

    matches!(
        response,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );

    // --- Verify that stake position was created ---
    let position = get_position(pic, user, gldt_stake_canister_id, &user);
    assert!(position.is_none());
}

#[test]
fn add_stake_new_position_zero() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers.get("gldt_ledger_canister_id").unwrap();

    // --- Create user and fund it ---
    let user = create_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        4_000_000_000_u128,
    );

    // --- Perform stake operation ---
    let response_first_stake = add_stake(
        pic,
        user,
        gldt_ledger_id,
        gldt_stake_canister_id,
        1_000_000_000_u128,
    );

    matches!(
        response_first_stake,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );

    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_secs(60 * 60 * 24 * 30)); // 30 days
    tick_n_blocks(pic, 10);

    // --- Stake for second time ---
    let response_second_stake =
        add_stake(pic, user, gldt_ledger_id, gldt_stake_canister_id, 0_u128);

    matches!(
        response_second_stake,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );
}

#[test]
fn add_stake_new_position_small_amount() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();
    let gldt_ledger_id = token_ledgers.get("gldt_ledger_canister_id").unwrap();

    // --- Create user and fund it ---
    let user = create_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        4_000_000_000_u128,
    );

    // --- Perform stake operation ---
    let response_first_stake = add_stake(
        pic,
        user,
        gldt_ledger_id,
        gldt_stake_canister_id,
        1_000_000_000_u128,
    );

    matches!(
        response_first_stake,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );

    // --- Stake for second time ---
    let response_second_stake =
        add_stake(pic, user, gldt_ledger_id, gldt_stake_canister_id, 1_u128);
    matches!(
        response_second_stake,
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::CallError(_)
        ))
    );
}
