use crate::client::gldt_stake::icrc3_get_blocks;
use crate::client::gldt_stake::{_set_position_withdraw_state, get_total_staked};
use crate::client::gldt_stake::{get_position, manage_stake_position};
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::{add_rewards_to_neurons, create_stake_position_util};
use crate::utils::wait_1_day;
use crate::{
    client::icrc1::client::balance_of, gldt_stake_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};
use assert_matches::assert_matches;
use candid::Nat;
use canister_time::{DAY_IN_MS, HOUR_IN_MS};
use gldt_stake_api_canister::manage_stake_position;
use gldt_stake_common::ledgers::GLDT_TX_FEE;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use gldt_stake_common::stake_position_event::{NormalWithdrawStatus, WithdrawState};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc3::blocks::GetBlocksRequest;
use std::time::Duration;
use types::TokenSymbol;

#[test]
fn test_withdraw_works() {
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
    let (user, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
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
    assert_eq!(total_staked_before_withdraw, Nat::from(1_000_000_000_u128));

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
    tick_n_blocks(pic, 100);

    let user_position = get_position(pic, user, gldt_stake_canister_id, &()).unwrap();
    let rewards = &user_position.claimable_rewards;
    assert_ne!(rewards[&TokenSymbol::GOLDAO], Nat::from(0_u64));
    assert_ne!(rewards[&TokenSymbol::OGY], Nat::from(0_u64));
    assert_ne!(rewards[&TokenSymbol::ICP], Nat::from(0_u64));

    let position_stake_amount = user_position.staked.clone();

    // --- Start dissolving ---
    let _ = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::StartDissolving { fraction: 100 },
    )
    .unwrap();

    // wait 1 day and try to withdraw - SHOULD FAIL because we haven't waited a full week
    pic.advance_time(Duration::from_millis(DAY_IN_MS));
    let res = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::Withdraw {},
    );

    assert_matches!(res, Err(ManageStakePositionError::WithdrawError(_)));

    // --- Claim all rewards ---
    let _res = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::ClaimRewards {
            tokens: vec![TokenSymbol::GOLDAO, TokenSymbol::OGY, TokenSymbol::ICP],
        },
    );

    // --- Check that the dissolve date hasn't passed ---
    let res = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::Withdraw {},
    );

    assert_matches!(res, Err(ManageStakePositionError::WithdrawError(_)));

    pic.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    tick_n_blocks(pic, 100);

    let res = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::Withdraw {},
    );

    let user_position = get_position(pic, user, gldt_stake_canister_id, &());
    assert!(user_position.is_some());

    tick_n_blocks(pic, 10);
    assert_matches!(res, Ok(_));
    let position = res.unwrap();
    assert_eq!(position.staked, Nat::from(0u64));

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

    assert_eq!(
        user_gldt_balance_after_withdraw,
        user_gldt_balance + position_stake_amount - GLDT_TX_FEE
    );

    // Check that the user position is deleted
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 6));

    pic.advance_time(Duration::from_millis(DAY_IN_MS * 8));
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 8));
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 8));
    tick_n_blocks(pic, 10);
    let user_position = get_position(pic, user, gldt_stake_canister_id, &());
    assert!(user_position.is_none());
}

#[test]
fn test_withdraw_with_unclaimed_rewards() {
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

    // --- Create stake position ---
    let (user, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );

    let total_staked_before_withdraw = get_total_staked(pic, user, gldt_stake_canister_id, &());
    assert_eq!(total_staked_before_withdraw, Nat::from(1_000_000_000_u128));

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

    let user_position = get_position(pic, user, gldt_stake_canister_id, &()).unwrap();
    let rewards = &user_position.claimable_rewards;
    assert_ne!(rewards[&TokenSymbol::GOLDAO], Nat::from(0_u64));
    assert_ne!(rewards[&TokenSymbol::OGY], Nat::from(0_u64));
    assert_ne!(rewards[&TokenSymbol::ICP], Nat::from(0_u64));

    // --- Start dissolving ---
    let _ = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::StartDissolving { fraction: 100 },
    )
    .unwrap();

    // --- Check that the dissolve date hasn't passed ---
    let res = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::Withdraw {},
    );

    assert_matches!(res, Err(ManageStakePositionError::WithdrawError(_)));
}

#[test]
fn test_invalid_withdraw_states_in_progress() {
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

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user, _stake_position) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );

    // start dissolving
    let _ = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::StartDissolving { fraction: 100 },
    )
    .unwrap();

    // wait 7 days
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 7));

    // force position into an InProgress state
    _set_position_withdraw_state(
        pic,
        controller,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::_set_position_withdraw_state::Args {
            principal: user,
            state: WithdrawState::NormalWithdraw(NormalWithdrawStatus::InProgress),
        },
    )
    .unwrap();
    pic.advance_time(Duration::from_secs(60));
    tick_n_blocks(pic, 10);

    // attempt to withdraw using normal API - it should fail because the position is already in progress.
    let res = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::Withdraw {},
    );
    println!("{res:?}");
    assert_matches!(res, Err(ManageStakePositionError::WithdrawError(_)));
}

#[test]
fn test_invalid_withdraw_states_failed() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();

    let (user, _stake_position) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );

    // --- Start dissolving ---
    let _ = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::StartDissolving { fraction: 100 },
    )
    .unwrap();

    // wait 7 days
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 7));

    // --- Force position into an Failed state ---
    _set_position_withdraw_state(
        pic,
        controller,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::_set_position_withdraw_state::Args {
            principal: user,
            state: WithdrawState::NormalWithdraw(NormalWithdrawStatus::Failed(format!(""))),
        },
    )
    .unwrap();
    pic.advance_time(Duration::from_secs(60));
    tick_n_blocks(pic, 10);

    // attempt to withdraw using normal API - it should fail because the position is already in progress.
    let res = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::Withdraw {},
    );
    println!("res: {:?}", res);
    // assert_eq!(res.is_ok(), true);

    let get_blocks_args = vec![GetBlocksRequest {
        start: Nat::from(0u64),
        length: Nat::from(100u64),
    }];
    let blocks = icrc3_get_blocks(pic, controller, gldt_stake_canister_id, &get_blocks_args);
    println!("blocks: {blocks:?}");
    assert_eq!(blocks.blocks.len(), 1);
    let archived_blocks_amount = u128::try_from(
        &blocks
            .archived_blocks
            .first()
            .unwrap()
            .args
            .first()
            .unwrap()
            .length
            .0,
    )
    .unwrap();
    assert_eq!(blocks.blocks.len() as u128 + archived_blocks_amount, 3);

    let user_position = get_position(pic, user, gldt_stake_canister_id, &());
    println!("user_position: {user_position:?}");
}
