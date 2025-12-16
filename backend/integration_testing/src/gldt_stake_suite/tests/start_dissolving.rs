use crate::client::gldt_stake::get_position;
use crate::client::gldt_stake::manage_stake_position_with_tick;
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::create_stake_position_util_mock;
use crate::{gldt_stake_suite::setup::default_test_setup, utils::tick_n_blocks};
use assert_matches::assert_matches;
use candid::Nat;
use gldt_stake_api_canister::manage_stake_position;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::manage_stake_position_interface::StartDissolvingErrors;

#[test]
fn test_start_dissolving() {
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

    // --- Create stake position ---
    let (user, _) = create_stake_position_util_mock(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        5_000_000_000u128,
    );

    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::StartDissolving { fraction: 100 },
    );
    assert!(matches!(response, Result::Ok(_)));

    tick_n_blocks(pic, 1);

    let user_position = get_position(pic, user, gldt_stake_canister_id, &user).unwrap();
    assert_eq!(user_position.staked, Nat::from(0_u64));
}

#[test]
fn test_start_dissolving_partial() {
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

    // --- Create stake position ---
    let (user, _) = create_stake_position_util_mock(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        100_000_000_000_u128,
    );

    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::StartDissolving { fraction: 50 },
    );
    assert!(matches!(response, Result::Ok(_)));

    tick_n_blocks(pic, 1);

    let user_position = get_position(pic, user, gldt_stake_canister_id, &user).unwrap();
    assert_eq!(user_position.staked, Nat::from(50_000_000_000_u128));
}

#[test]
fn test_start_dissolving_limit() {
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

    // --- Create stake position ---
    let (user, _) = create_stake_position_util_mock(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        100_000_000_000u128,
    );

    // --- Create 5 legit dissolvements ---
    for _ in 0..=4 {
        let response = manage_stake_position_with_tick(
            pic,
            user,
            gldt_stake_canister_id,
            &manage_stake_position::Args::StartDissolving { fraction: 20 },
        );

        assert!(response.is_ok());
    }

    tick_n_blocks(pic, 1);

    // --- Create the 6th not legit dissolvements ---
    let response = manage_stake_position_with_tick(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::StartDissolving { fraction: 10 },
    );

    assert_matches!(
        response,
        Err(ManageStakePositionError::StartDissolvingError(
            StartDissolvingErrors::DissolvementsLimitReached(_)
        ))
    );

    let user_position = get_position(pic, user, gldt_stake_canister_id, &user).unwrap();
    assert_eq!(user_position.staked, Nat::from(32_768_000_000_u128));
}
