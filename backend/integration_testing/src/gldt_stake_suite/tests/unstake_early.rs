use assert_matches::assert_matches;
use candid::{CandidType, Deserialize};
use candid::{Nat, Principal};
use canister_time::{DAY_IN_MS, HOUR_IN_MS};
use gldt_stake_api_canister::{get_historic_positions_by_user, unstake_early};
use gldt_stake_common::accounts::EARLY_UNSTAKE_FEE_ACCOUNT;
use gldt_stake_common::ledgers::GLDT_TX_FEE;
use gldt_stake_common::stake_position::{UnstakeEarlyRequestErrors, UnstakeErrors};
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use std::time::Duration;

use crate::client::gldt_stake::{
    claim_reward, get_active_user_positions, get_historic_position_by_id,
    get_historic_positions_by_user, get_total_staked, start_dissolving, unstake_early,
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
fn test_unstake_early() {
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
    let position_early_unstake_fee = user_0_positions.get(0).unwrap().early_unstake_fee.clone();

    // wait the remaining 6 days
    pic_borrowed.advance_time(Duration::from_millis(DAY_IN_MS * 6));

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

    let res = unstake_early(pic_borrowed, user_0, gldt_stake_canister_id, &position_id);
    tick_n_blocks(pic_borrowed, 5);
    assert_matches!(res, Ok(_));
    let position = res.unwrap();
    assert_eq!(position.staked, Nat::from(0u64));
    tick_n_blocks(pic_borrowed, 2);

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
        (user_gldt_balance + position_stake_amount)
            - (position_early_unstake_fee.clone() + GLDT_TX_FEE)
    );

    // wait one hour for the fees to be transferred
    pic_borrowed.advance_time(Duration::from_millis(HOUR_IN_MS));
    tick_n_blocks(pic_borrowed, 2);

    let fee_account_balance = balance_of(
        pic_borrowed,
        gldt_ledger_id.clone(),
        Account {
            owner: gldt_stake_canister_id,
            subaccount: Some(EARLY_UNSTAKE_FEE_ACCOUNT),
        },
    );

    assert_eq!(
        fee_account_balance,
        position_early_unstake_fee - GLDT_TX_FEE
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
fn test_unstake_early_when_position_is_dissolving() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;
    let pic_borrowed = &pic.borrow();

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

    let res = unstake_early(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &stake_position.id,
    );

    println!("{res:?}");
    assert_matches!(
        res,
        unstake_early::Response::Err(UnstakeEarlyRequestErrors::UnstakeErrors(
            UnstakeErrors::InvalidDissolveState(_)
        ))
    );
}
