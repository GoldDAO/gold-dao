use crate::client::gldt_stake::add_whitelisted_principal;
use crate::client::gldt_stake::get_position;
use crate::client::gldt_stake::manage_stake_position;
use crate::client::gldt_stake::*;
use crate::client::pocket::unwrap_response;
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::{add_rewards_to_neurons, create_stake_position_util};
use crate::utils::wait_1_day;
use crate::{
    client::icrc1::client::balance_of, gldt_stake_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};
use assert_matches::assert_matches;
use candid::{Encode, Nat, Principal};
use canister_time::HOUR_IN_MS;
use gldt_stake_api_canister::manage_stake_position;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionError;
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;
use types::TokenSymbol;

#[test]
fn test_can_claim_gldt_stake_rewards() {
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
    let goldao_ledger = token_ledgers.get("goldao_ledger_canister_id").unwrap();
    let goldao_tx_fee = ledger_fees.get("GOLDAO").unwrap();

    // --- Create stake position ---
    let (user, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );

    // --- Add rewards to neurons ---
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

    let unallocated_rewards =
        unallocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Unallocated rewards balance: {:?}", unallocated_rewards);

    let processing_rewards =
        processing_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Processing rewards balance: {:?}", processing_rewards);

    let allocated_rewards =
        allocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Allocated rewards balance: {:?}", allocated_rewards);

    // --- Check that the rewards are available for the user ---
    let user_position = get_position(pic, user, gldt_stake_canister_id, &()).unwrap();
    let rewards = &user_position.claimable_rewards;
    assert_eq!(rewards[&TokenSymbol::GOLDAO], Nat::from(47_142_757_142_u64));
    assert_eq!(rewards[&TokenSymbol::OGY], Nat::from(47_142_657_142_u64));
    assert_eq!(rewards[&TokenSymbol::ICP], Nat::from(47_142_847_142_u64));

    pic.advance_time(Duration::from_secs(2));
    tick_n_blocks(pic, 50);

    // --- Claim rewards ---
    let expected_reward = user_position
        .claimable_rewards
        .get(&TokenSymbol::GOLDAO)
        .unwrap()
        .clone()
        - goldao_tx_fee.clone();
    let res = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id.clone(),
        &manage_stake_position::Args::ClaimRewards {
            tokens: vec![TokenSymbol::GOLDAO],
        },
    )
    .unwrap();

    // --- Check result of claim rewards ---
    assert_eq!(
        res.claimable_rewards.get(&TokenSymbol::GOLDAO).unwrap(),
        &Nat::from(0u64)
    );
    let user_goldao_balance = balance_of(
        pic,
        goldao_ledger.clone(),
        Account {
            owner: user,
            subaccount: None,
        },
    );
    assert_eq!(user_goldao_balance, expected_reward);

    // --- Doublecheck that the state contains the updated stake position ---
    let position = get_position(pic, user, gldt_stake_canister_id, &())
        .unwrap()
        .clone();
    assert_eq!(
        position
            .claimable_rewards
            .get(&TokenSymbol::GOLDAO)
            .unwrap()
            .clone(),
        Nat::from(0u64)
    );

    let total_allocated_rewards =
        get_total_allocated_rewards(pic, user, gldt_stake_canister_id, &());
    println!("total_allocated_rewards: {:?}", total_allocated_rewards);
}

#[test]
fn test_claim_rewards_guards_as_anonymous_principal() {
    // --- Setup test environment ---
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic = &pic.borrow();

    let _ = add_whitelisted_principal(
        pic,
        controller,
        gldt_stake_canister_id,
        &vec![Principal::anonymous()],
    );

    // --- Check that calls with anonymous callers are rejected ---
    let res = manage_stake_position(
        pic,
        Principal::anonymous(),
        gldt_stake_canister_id.clone(),
        &manage_stake_position::Args::ClaimRewards {
            tokens: vec![TokenSymbol::GOLDAO],
        },
    );

    assert_matches!(
        res,
        Err(ManageStakePositionError::GeneralError(
            GeneralError::InvalidPrincipal(_)
        ))
    );
}

#[test]
fn test_claim_rewards_twice() {
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
        1_000_000_000_u128,
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

    let user_position = get_position(pic, user, gldt_stake_canister_id, &()).unwrap();
    let rewards = &user_position.claimable_rewards;
    assert_eq!(rewards[&TokenSymbol::GOLDAO], Nat::from(47_142_757_142_u64));
    assert_eq!(rewards[&TokenSymbol::OGY], Nat::from(47_142_657_142_u64));
    assert_eq!(rewards[&TokenSymbol::ICP], Nat::from(47_142_847_142_u64));

    let _res = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::ClaimRewards {
            tokens: vec![TokenSymbol::GOLDAO],
        },
    );
    let res = manage_stake_position(
        pic,
        user,
        gldt_stake_canister_id,
        &manage_stake_position::Args::ClaimRewards {
            tokens: vec![TokenSymbol::GOLDAO],
        },
    );

    assert_matches!(res, Err(ManageStakePositionError::ClaimRewardError(_)));
}

#[test]
fn test_claim_rewards_concurrent_calls_should_fail() {
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

    let unallocated_rewards =
        unallocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Unallocated rewards balance: {:?}", unallocated_rewards);

    let processing_rewards =
        processing_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Processing rewards balance: {:?}", processing_rewards);

    let allocated_rewards =
        allocated_rewards_balance(pic, controller, gldt_stake_canister_id.clone(), &());
    println!("Allocated rewards balance: {:?}", allocated_rewards);

    let user_position = get_position(pic, user, gldt_stake_canister_id, &()).unwrap();
    let rewards = &user_position.claimable_rewards;
    assert_eq!(rewards[&TokenSymbol::GOLDAO], Nat::from(47_142_757_142_u64));
    assert_eq!(rewards[&TokenSymbol::OGY], Nat::from(47_142_657_142_u64));
    assert_eq!(rewards[&TokenSymbol::ICP], Nat::from(47_142_847_142_u64));

    // --- Create two parallel calls for one position ---
    let message_id_1 = pic
        .submit_call(
            gldt_stake_canister_id,
            user,
            "manage_stake_position",
            Encode!(&manage_stake_position::Args::ClaimRewards {
                tokens: vec![TokenSymbol::GOLDAO],
            })
            .unwrap(),
        )
        .unwrap();
    let message_id_2 = pic
        .submit_call(
            gldt_stake_canister_id,
            user,
            "manage_stake_position",
            Encode!(&manage_stake_position::Args::ClaimRewards {
                tokens: vec![TokenSymbol::GOLDAO],
            })
            .unwrap(),
        )
        .unwrap();

    let res_1 = pic.await_call(message_id_1);
    let res_1: manage_stake_position::Response = unwrap_response(res_1);

    let res_2 = pic.await_call(message_id_2);
    let res_2: manage_stake_position::Response = unwrap_response(res_2);

    println!("{res_1:?}");
    println!("{res_2:?}");

    // --- Check that one of the calls is rejected ---
    match res_1 {
        Ok(_) => {
            assert_matches!(
                res_2,
                Err(ManageStakePositionError::GeneralError(
                    GeneralError::AlreadyProcessing(_)
                ))
            );
        }
        Err(_) => {
            assert_eq!(matches!(res_2, Ok(_)), true);
        }
    }
}
