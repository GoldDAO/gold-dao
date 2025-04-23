use candid::{Encode, Nat, Principal};
use canister_time::{DAY_IN_MS, HOUR_IN_MS};
use gldt_stake_api_canister::claim_reward;
use gldt_stake_common::stake_position::ClaimRewardErrors;
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;

use crate::client::gldt_stake::{claim_reward, get_active_user_positions};
use crate::client::pocket::unwrap_response;
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::{add_rewards_to_neurons, create_stake_position_util};
use crate::{
    client::icrc1::client::balance_of, gldt_stake_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};

#[test]
fn test_can_claim_gldt_stake_rewards() {
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

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, stake_position) = create_stake_position_util(
        pic_borrowed,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
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

    // claim rewards
    let goldao_ledger = token_ledgers.get("goldao_ledger_canister_id").unwrap();
    let goldao_tx_fee = ledger_fees.get("GOLDAO").unwrap();
    let expected_reward = user_0_positions
        .get(0)
        .unwrap()
        .claimable_rewards
        .get("GOLDAO")
        .unwrap()
        .clone()
        - goldao_tx_fee.clone();

    let res = claim_reward(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::claim_reward::Args {
            id: position_id,
            token: "GOLDAO".to_string(),
        },
    )
    .unwrap();

    assert_eq!(
        res.claimable_rewards.get("GOLDAO").unwrap(),
        &Nat::from(0u64)
    );
    let user_goldao_balance = balance_of(
        pic_borrowed,
        goldao_ledger.clone(),
        Account {
            owner: user_0,
            subaccount: None,
        },
    );
    assert_eq!(user_goldao_balance, expected_reward);

    // get user position to double check it was saved to state
    let position = get_active_user_positions(pic_borrowed, user_0, gldt_stake_canister_id, &None)
        .get(0)
        .unwrap()
        .clone();
    assert_eq!(
        position.claimable_rewards.get("GOLDAO").unwrap().clone(),
        Nat::from(0u64)
    );
}

#[test]
fn test_claim_rewards_guards_as_anonymous_principal() {
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

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, stake_position) = create_stake_position_util(
        pic_borrowed,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
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

    // test annoymous principal - should error
    let res = claim_reward(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &gldt_stake_api_canister::claim_reward::Args {
            id: position_id,
            token: "GOLDAO".to_string(),
        },
    );

    assert_eq!(
        matches!(res, Err(ClaimRewardErrors::InvalidPrincipal(_))),
        true
    );
}

#[test]
// #[should_panic]
fn test_claim_rewards_after_successful_claim() {
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

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, stake_position) = create_stake_position_util(
        pic_borrowed,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
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

    let _ = claim_reward(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::claim_reward::Args {
            id: position_id,
            token: "GOLDAO".to_string(),
        },
    );
    let res = claim_reward(
        pic_borrowed,
        user_0,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::claim_reward::Args {
            id: position_id,
            token: "GOLDAO".to_string(),
        },
    );

    assert_eq!(
        matches!(res, Err(ClaimRewardErrors::TokenImbalance(_))),
        true
    );
}

#[test]
// #[should_panic]
fn test_claim_rewards_duplicate_calls_should_fail() {
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

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, stake_position) = create_stake_position_util(
        pic_borrowed,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
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

    let message_id_1 = pic_borrowed
        .submit_call(
            gldt_stake_canister_id,
            user_0,
            "claim_reward",
            Encode!(&gldt_stake_api_canister::claim_reward::Args {
                id: position_id,
                token: "GOLDAO".to_string(),
            })
            .unwrap(),
        )
        .unwrap();
    let message_id_2 = pic_borrowed
        .submit_call(
            gldt_stake_canister_id,
            user_0,
            "claim_reward",
            Encode!(&gldt_stake_api_canister::claim_reward::Args {
                id: position_id,
                token: "GOLDAO".to_string(),
            })
            .unwrap(),
        )
        .unwrap();

    let res_1 = pic_borrowed.await_call(message_id_1);
    let res_1: claim_reward::Response = unwrap_response(res_1);

    let res_2 = pic_borrowed.await_call(message_id_2);
    let res_2: claim_reward::Response = unwrap_response(res_2);

    println!("{res_1:?}");
    println!("{res_2:?}");

    match res_1 {
        Ok(_) => {
            assert_eq!(
                matches!(res_2, Err(ClaimRewardErrors::AlreadyProcessing(_))),
                true
            );
        }
        Err(_) => {
            assert_eq!(matches!(res_2, Ok(_)), true);
        }
    }
}
