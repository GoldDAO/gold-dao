use assert_matches::assert_matches;
use candid::{CandidType, Deserialize};
use candid::{Nat, Principal};
use canister_time::{DAY_IN_MS, HOUR_IN_MS};
use gldt_stake_api_canister::unstake;
use gldt_stake_common::ledgers::GLDT_TX_FEE;
use gldt_stake_common::stake_position::{DissolveState, UnstakeErrors, UnstakeRequestErrors};
use gldt_stake_common::stake_position_event::{NormalUnstakeStatus, UnstakeState};
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use std::time::Duration;

use crate::client::gldt_stake::{
    _set_position_unstake_state, claim_reward, get_active_user_positions,
    get_historic_position_by_id, get_total_staked, start_dissolving, unstake,
};
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::{add_rewards_to_neurons, create_stake_position_util};
use crate::{
    client::icrc1::client::balance_of, gldt_stake_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_unstake() {
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
    let pic_borrowed = &pic.borrow();

    let gldt_ledger_id = token_ledgers.get("gldt_ledger_canister_id").unwrap();
    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, _) = create_stake_position_util(
        pic_borrowed,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );

    let user_gldt_balance = balance_of(
        pic_borrowed,
        gldt_ledger_id.clone(),
        Account {
            owner: user_0,
            subaccount: None,
        },
    );

    // ---------------------------------------
    //              W E E K   0
    // ---------------------------------------
    // wait for reward allocation to process
    // 10,000 GOLDAO, OGY and ICP will be given to user_0 because that is the only position available to allocate rewards to.

    add_rewards_to_neurons(
        pic_borrowed,
        neuron_data.clone(),
        controller,
        &token_ledgers,
        gld_rewards_canister_id,
        gldt_stake_canister_id,
        ledger_fees.clone(),
    );

    pic_borrowed.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    tick_n_blocks(pic_borrowed, 5);
    pic_borrowed.advance_time(Duration::from_millis(HOUR_IN_MS));
    tick_n_blocks(pic_borrowed, 5);

    let user_0_positions =
        get_active_user_positions(pic_borrowed, user_0, gldt_stake_canister_id, &None);
    println!("{user_0_positions:?}");
    user_0_positions
        .get(0)
        .unwrap()
        .claimable_rewards
        .iter()
        .for_each(|(_, reward)| {
            assert_eq!(reward, &Nat::from(1_000_000_000_000u64)); // 10,000 of each token type
        });
    let position_id = user_0_positions.get(0).unwrap().id;
    let position_stake_amount = user_0_positions.get(0).unwrap().staked.clone();

    // start dissolving
    let response =
        start_dissolving(pic_borrowed, user_0, gldt_stake_canister_id, &position_id).unwrap();
    assert_eq!(response.dissolve_state, DissolveState::Dissolving);

    // wait 1 day and try to unstake - SHOULD FAIL because we haven't waited a full 7 days
    pic_borrowed.advance_time(Duration::from_millis(DAY_IN_MS));
    let res = unstake(pic_borrowed, user_0, gldt_stake_canister_id, &position_id);

    assert_matches!(
        res,
        Err(UnstakeRequestErrors::UnstakeErrors(
            UnstakeErrors::DissolveDateNotSatisfied(_)
        ))
    );

    // wait the remaining 6 days
    pic_borrowed.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    let res = unstake(pic_borrowed, user_0, gldt_stake_canister_id, &position_id);

    assert_matches!(
        res,
        Err(UnstakeRequestErrors::UnstakeErrors(
            UnstakeErrors::CantUnstakeWithRewardsBalance(_)
        ))
    );

    // // claim rewards
    let _ = claim_reward(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::claim_reward::Args {
            id: position_id,
            token: "GOLDAO".to_string(),
        },
    )
    .unwrap();
    let _ = claim_reward(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::claim_reward::Args {
            id: position_id,
            token: "OGY".to_string(),
        },
    )
    .unwrap();
    let _ = claim_reward(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::claim_reward::Args {
            id: position_id,
            token: "ICP".to_string(),
        },
    )
    .unwrap();

    let res = unstake(pic_borrowed, user_0, gldt_stake_canister_id, &position_id);
    tick_n_blocks(pic_borrowed, 5);
    assert_matches!(res, Ok(_));
    let position = res.unwrap();
    assert_eq!(position.staked, Nat::from(0u64));

    let user_gldt_balance_after_unstake = balance_of(
        pic_borrowed,
        gldt_ledger_id.clone(),
        Account {
            owner: user_0,
            subaccount: None,
        },
    );

    assert_eq!(
        user_gldt_balance_after_unstake,
        user_gldt_balance + position_stake_amount - GLDT_TX_FEE
    );

    let total_staked = get_total_staked(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &(),
    );
    assert_eq!(total_staked, Nat::from(0u64));

    // check the position was moved to history
    tick_n_blocks(pic_borrowed, 5);
    let res = get_historic_position_by_id(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &position_id,
    )
    .unwrap();
    assert_eq!(res.id, position_id);

    let user_0_positions =
        get_active_user_positions(pic_borrowed, user_0, gldt_stake_canister_id, &None);
    assert_eq!(user_0_positions.len(), 0);
}

#[test]
fn test_invalid_unstake_states_in_progress() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic_borrowed = &pic.borrow();

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, stake_position) = create_stake_position_util(
        pic_borrowed,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );

    // start dissolving
    let _ = start_dissolving(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &stake_position.id,
    )
    .unwrap();

    // wait 7 days
    pic_borrowed.advance_time(Duration::from_millis(DAY_IN_MS * 7));

    // force position into an InProgress state
    _set_position_unstake_state(
        pic_borrowed,
        controller,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::_set_position_unstake_state::Args {
            id: stake_position.id,
            state: UnstakeState::NormalUnstake(NormalUnstakeStatus::InProgress),
        },
    )
    .unwrap();

    // attempt to unstake using normal API - it should fail because the position is already in progress.
    let res = unstake(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &stake_position.id,
    );
    println!("{res:?}");
    assert_eq!(
        matches!(
            res,
            Err(UnstakeRequestErrors::UnstakeErrors(
                UnstakeErrors::AlreadyProcessing(_)
            ))
        ),
        true
    )
}

#[test]
fn test_invalid_unstake_states_failed() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic_borrowed = &pic.borrow();

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, stake_position) = create_stake_position_util(
        pic_borrowed,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );

    // start dissolving
    let _ = start_dissolving(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &stake_position.id,
    )
    .unwrap();

    // wait 7 days
    pic_borrowed.advance_time(Duration::from_millis(DAY_IN_MS * 7));

    // force position into an InProgress state
    _set_position_unstake_state(
        pic_borrowed,
        controller,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::_set_position_unstake_state::Args {
            id: stake_position.id,
            state: UnstakeState::NormalUnstake(NormalUnstakeStatus::Failed(format!(""))),
        },
    )
    .unwrap();

    // attempt to unstake using normal API - it should fail because the position is already in progress.
    let res = unstake(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &stake_position.id,
    );
    assert_eq!(res.is_ok(), true);
}

#[test]
fn test_invalid_unstake_states_unstaked() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic_borrowed = &pic.borrow();

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, stake_position) = create_stake_position_util(
        pic_borrowed,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );

    // start dissolving
    let _ = start_dissolving(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &stake_position.id,
    )
    .unwrap();

    // wait 7 days
    pic_borrowed.advance_time(Duration::from_millis(DAY_IN_MS * 7));

    // force position into an InProgress state
    _set_position_unstake_state(
        pic_borrowed,
        controller,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::_set_position_unstake_state::Args {
            id: stake_position.id,
            state: UnstakeState::NormalUnstake(NormalUnstakeStatus::Unstaked),
        },
    )
    .unwrap();

    // attempt to unstake using normal API - it should fail because the position is already in progress.
    let res = unstake(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &stake_position.id,
    );
    println!("{res:?}");
    assert_matches!(
        res,
        unstake::Response::Err(UnstakeRequestErrors::UnstakeErrors(
            UnstakeErrors::AlreadyUnstaked(_)
        ))
    );
}
