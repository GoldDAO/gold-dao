use crate::client::gldt_stake::_add_whitelisted_principal;
use crate::client::gldt_stake::get_total_staked;
use crate::gldt_stake_suite::utils::add_stake;
use crate::gldt_stake_suite::utils::create_whitelisted_user_with_funds;
use crate::{
    client::{
        gldt_stake::{get_position, manage_stake_position},
        icrc1_icrc2_token::icrc2_approve,
    },
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
    let user = create_whitelisted_user_with_funds(
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
    let response = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id.clone(),
        &manage_stake_position::Args::AddStake {
            amount: Nat::from(total_approval),
        },
    )
    .expect("Failed to add stake");

    assert_eq!(response.staked, Nat::from(stake_amount));
    assert_eq!(response.age_bonus_multiplier, 1.0);

    // --- Verify that stake position was created ---
    let position = get_position(pic, user, gldt_stake_canister_id, &());
    assert!(position.is_some(), "Stake position should exist");
}

#[test]
fn add_stake_existing_position_works() {
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
    let user = create_whitelisted_user_with_funds(
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
    )
    .expect("Failed to add stake");

    println!(
        "age bonus after 1 stake: {}",
        response_first_stake.age_bonus_multiplier
    );
    assert_eq!(response_first_stake.staked, Nat::from(1_000_000_000_u64));
    assert_eq!(response_first_stake.age_bonus_multiplier, 1.0);

    // --- Check stake system paramters ---
    let total_staked = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(total_staked, Nat::from(1_000_000_000_u64));

    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_secs(60 * 60 * 24 * 30)); // 30 days
    tick_n_blocks(pic, 10);

    // --- Stake for second time ---
    let response_second_stake = add_stake(
        pic,
        user,
        gldt_ledger_id,
        gldt_stake_canister_id,
        1_500_000_000_u128,
    )
    .expect("Failed to add stake for the second time");

    println!(
        "age bonus after 2 stake: {}",
        response_second_stake.age_bonus_multiplier
    );
    assert_eq!(response_second_stake.staked, Nat::from(2_500_000_000_u64));
    assert!(response_second_stake.age_bonus_multiplier > response_first_stake.age_bonus_multiplier);
    assert!(response_second_stake.age_bonus_multiplier > 1.0);

    // --- Check stake system paramters ---
    let total_staked_after = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(total_staked_after, Nat::from(2_500_000_000_u64));
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

    let _ = _add_whitelisted_principal(
        pic,
        controller,
        gldt_stake_canister_id,
        &vec![Principal::anonymous()],
    );

    // --- Stake with anonymous caller ---
    let amount_to_add = 10_000_000_000;
    let res = manage_stake_position(
        pic,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &manage_stake_position::Args::AddStake {
            amount: Nat::from(amount_to_add + GLDT_TX_FEE as u128),
        },
    );

    match res {
        Ok(_) => panic!("Expected an error, but got a success response"),
        Err(ManageStakePositionError::GeneralError(GeneralError::InvalidPrincipal(e))) => {
            println!("Invalid principal: {:?}", e);
        }
        Err(_) => {
            panic!("Expected a particular error, but got another response");
        }
    }
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

    let user = create_whitelisted_user_with_funds(
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

    let is_err1 = matches!(
        result1,
        Err(ManageStakePositionError::GeneralError(
            GeneralError::AlreadyProcessing(_)
        ))
    );
    let is_err2 = matches!(
        result2,
        Err(ManageStakePositionError::GeneralError(
            GeneralError::AlreadyProcessing(_)
        ))
    );

    assert!(
        is_err1 ^ is_err2,
        "Expected exactly one error of AlreadyProcessing, got result1: {:?}, result2: {:?}",
        result1,
        result2
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
    let user = create_whitelisted_user_with_funds(
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

    match response {
        Ok(_) => panic!("Expected an error, but got a success response"),
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::InvalidStakeAmount(e),
        )) => {
            println!("{:?}", e);
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
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
    let user = create_whitelisted_user_with_funds(
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

    match response {
        Ok(_) => panic!("Expected an error, but got a success response"),
        Err(ManageStakePositionError::AddStakeError(
            AddStakePositionErrors::InvalidStakeAmount(e),
        )) => {
            println!("{:?}", e);
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
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
    let user = create_whitelisted_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        2_000_000_000u128,
    );

    // --- Perform stake operation ---
    let stake_amount = 1_000_000_000u128;
    let total_approval = stake_amount + GLDT_TX_FEE as u128;
    let response = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id.clone(),
        &manage_stake_position::Args::AddStake {
            amount: Nat::from(total_approval),
        },
    );

    match response {
        Ok(_) => panic!("Expected an error, but got a success response"),
        Err(ManageStakePositionError::GeneralError(GeneralError::TransferError(e))) => {
            assert!(e.contains("InsufficientAllowance"));
            println!("{:?}", e);
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // --- Verify that stake position was created ---
    let position = get_position(pic, user, gldt_stake_canister_id, &());
    assert!(position.is_none());
}

#[test]
fn add_stake_new_position_after_dissolving() {
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
    let user = create_whitelisted_user_with_funds(
        pic,
        controller,
        gldt_stake_canister_id,
        gldt_ledger_id,
        3_000_000_000_u128,
    );

    // --- Approve stake amount for spender (gldt_stake_canister) ---
    let stake_amount = 1_000_000_000_u128;
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
    let response = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id.clone(),
        &manage_stake_position::Args::AddStake {
            amount: Nat::from(total_approval),
        },
    )
    .expect("Failed to add stake");

    assert_eq!(response.staked, Nat::from(stake_amount));
    assert_eq!(response.age_bonus_multiplier, 1.0);

    // --- Dissolve position ---
    let response = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::DissolveInstantly { fraction: 100 },
    )
    .expect("Failed to dissolve position");
    println!("Dissolve response: {:?}", response);

    // --- Add stake to dissolved position ---
    let _approval_result = icrc2_approve(
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

    let response = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id.clone(),
        &manage_stake_position::Args::AddStake {
            amount: Nat::from(total_approval),
        },
    )
    .expect("Failed to add stake");
    println!("Add stake response: {:?}", response);

    assert_eq!(response.staked, Nat::from(stake_amount));
    assert_eq!(response.age_bonus_multiplier, 1.0);

    // --- Verify that stake position was created ---
    let position = get_position(pic, user, gldt_stake_canister_id, &());
    assert!(position.is_some(), "Stake position should exist");
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
    let user = create_whitelisted_user_with_funds(
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
    )
    .expect("Failed to add stake");

    println!(
        "age bonus after 1 stake: {}",
        response_first_stake.age_bonus_multiplier
    );
    assert_eq!(response_first_stake.staked, Nat::from(1_000_000_000_u64));
    assert_eq!(response_first_stake.age_bonus_multiplier, 1.0);

    // --- Check stake system paramters ---
    let total_staked = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(total_staked, Nat::from(1_000_000_000_u64));

    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_secs(60 * 60 * 24 * 30)); // 30 days
    tick_n_blocks(pic, 10);

    // --- Stake for second time ---
    let response_second_stake =
        add_stake(pic, user, gldt_ledger_id, gldt_stake_canister_id, 0_u128);

    match response_second_stake {
        Ok(_) => panic!("Expected an error, but got a success response"),
        Err(ManageStakePositionError::GeneralError(GeneralError::ModifyStakeError(e))) => {
            println!("{:?}", e);
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // --- Check stake system paramters ---
    let total_staked_after = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(total_staked_after, Nat::from(1_000_000_000_u64));
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
    let user = create_whitelisted_user_with_funds(
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
    )
    .expect("Failed to add stake");

    println!(
        "age bonus after 1 stake: {}",
        response_first_stake.age_bonus_multiplier
    );
    assert_eq!(response_first_stake.staked, Nat::from(1_000_000_000_u64));
    assert_eq!(response_first_stake.age_bonus_multiplier, 1.0);

    // --- Check stake system paramters ---
    let total_staked = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(total_staked, Nat::from(1_000_000_000_u64));

    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_secs(60 * 60 * 24 * 30)); // 30 days
    tick_n_blocks(pic, 10);

    // --- Stake for second time ---
    let response_second_stake =
        add_stake(pic, user, gldt_ledger_id, gldt_stake_canister_id, 1_u128);

    match response_second_stake {
        Ok(_) => panic!("Expected an error, but got a success response"),
        Err(ManageStakePositionError::GeneralError(GeneralError::TransferError(e))) => {
            println!("{:?}", e);
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // --- Check stake system paramters ---
    let total_staked_after = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(total_staked_after, Nat::from(1_000_000_000_u64));
}
