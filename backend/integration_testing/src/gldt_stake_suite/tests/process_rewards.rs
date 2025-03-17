use assert_matches::assert_matches;
use candid::{CandidType, Deserialize};
use candid::{Nat, Principal};
use canister_time::{DAY_IN_MS, HOUR_IN_MS};
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use std::collections::HashMap;
use std::time::Duration;

use crate::client::gldt_stake::{
    _add_reward_round, get_active_user_positions, get_reward_rounds, get_total_allocated_rewards,
    process_oldest_reward_round, start_dissolving,
};
use crate::client::icrc1::client::transfer;
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::{add_rewards_to_neurons, create_stake_position_util};
use crate::{gldt_stake_suite::setup::default_test_setup, utils::tick_n_blocks};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn process_staking_rewards_works() {
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

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let (user_1, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let (user_2, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let (user_3, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let (user_4, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let (user_5, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let (user_6, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let (user_7, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let (user_8, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let (user_9, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let all_users = vec![
        user_0, user_1, user_2, user_3, user_4, user_5, user_6, user_7, user_8, user_9,
    ];

    // ---------------------------------------
    //              W E E K   0
    // ---------------------------------------
    // wait for reward allocation to process
    // we expect each stake position to receive 1,000 GOLDAO because there are 10 stake positions each with a 10% share of the GLDT Stake pool
    // we setup the environment so that the date is friday which means each position will still have an age bonus of 1.0 after advancing 6 days, see docs on how age bonus advances

    add_rewards_to_neurons(
        pic,
        neuron_data.clone(),
        controller,
        &token_ledgers,
        gld_rewards_canister_id,
        gldt_stake_canister_id,
        ledger_fees.clone(),
    );

    // first distribution
    pic.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    tick_n_blocks(pic, 5);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS));
    tick_n_blocks(pic, 5);

    all_users.iter().for_each(|user| {
        let positions =
            get_active_user_positions(pic, user.clone(), gldt_stake_canister_id, &(None));
        assert_eq!(positions.len(), 1);

        assert_eq!(
            positions
                .get(0)
                .unwrap()
                .claimable_rewards
                .get("GOLDAO")
                .unwrap(),
            &Nat::from(100_000_000_000u64)
        );

        assert_eq!(
            positions
                .get(0)
                .unwrap()
                .claimable_rewards
                .get("OGY")
                .unwrap(),
            &Nat::from(100_000_000_000u64)
        );

        assert_eq!(
            positions
                .get(0)
                .unwrap()
                .claimable_rewards
                .get("ICP")
                .unwrap(),
            &Nat::from(100_000_000_000u64)
        );
    });
}

#[test]
fn test_only_non_dissolving_positions_receive_rewards() {
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

    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let (user_0, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let (user_1, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );

    // begin dissolving user_1's position, this will mean user_0 will get all the rewards
    let user_1_positions = get_active_user_positions(pic, user_1, gldt_stake_canister_id, &None);
    let position_id = user_1_positions.get(0).unwrap().id;
    let _ = start_dissolving(pic, user_1, gldt_stake_canister_id, &position_id);
    tick_n_blocks(pic, 1);

    // ---------------------------------------
    //              W E E K   0
    // ---------------------------------------
    // wait for reward allocation to process
    // only one position will be eligble for rewards and it's bonus will be 1.0 because not enough time has passed for it's age bonus to increase

    add_rewards_to_neurons(
        pic,
        neuron_data.clone(),
        controller,
        &token_ledgers,
        gld_rewards_canister_id,
        gldt_stake_canister_id,
        ledger_fees.clone(),
    );

    pic.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    tick_n_blocks(pic, 5);
    pic.advance_time(Duration::from_millis(HOUR_IN_MS));
    tick_n_blocks(pic, 5);

    let user_1_positions = get_active_user_positions(pic, user_1, gldt_stake_canister_id, &None);
    user_1_positions
        .get(0)
        .unwrap()
        .claimable_rewards
        .iter()
        .for_each(|(_, reward)| {
            assert_eq!(reward, &Nat::from(0u64));
        });

    let user_0_positions = get_active_user_positions(pic, user_0, gldt_stake_canister_id, &None);
    user_0_positions
        .get(0)
        .unwrap()
        .claimable_rewards
        .iter()
        .for_each(|(_, reward)| {
            assert_eq!(reward, &Nat::from(1_000_000_000_000u64)); // 10,000 of each token type
        });
}

#[test]
fn test_processing_faulty_rounds() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;

    let (user_0, _) = create_stake_position_util(
        pic,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        1_000_000_000u128,
    );
    let goldao_ledger = token_ledgers
        .get("goldao_ledger_canister_id")
        .unwrap()
        .clone();

    // add first round
    let amount_1 = 1_000_000_000u128;
    transfer(
        pic,
        controller,
        goldao_ledger,
        None,
        Account {
            owner: gldt_stake_canister_id,
            subaccount: None,
        },
        amount_1,
    )
    .unwrap();
    let mut rewards = HashMap::new();
    rewards.insert("GOLDAO".to_string(), Nat::from(amount_1));
    _add_reward_round(pic, controller, gldt_stake_canister_id, &rewards).unwrap();

    let current_reward_rounds =
        get_reward_rounds(pic, Principal::anonymous(), gldt_stake_canister_id, &());
    assert_eq!(current_reward_rounds.len(), 1);

    // add second round
    let amount_2 = 2_000_000_000u128;
    transfer(
        pic,
        controller,
        goldao_ledger,
        None,
        Account {
            owner: gldt_stake_canister_id,
            subaccount: None,
        },
        amount_2,
    )
    .unwrap();
    let mut rewards = HashMap::new();
    rewards.insert("GOLDAO".to_string(), Nat::from(amount_2));
    _add_reward_round(pic, controller, gldt_stake_canister_id, &rewards).unwrap();

    let current_reward_rounds =
        get_reward_rounds(pic, Principal::anonymous(), gldt_stake_canister_id, &());
    assert_eq!(current_reward_rounds.len(), 2);

    // process rounds
    process_oldest_reward_round(pic, controller, gldt_stake_canister_id, &()).unwrap();
    let user_0_positions = get_active_user_positions(pic, user_0, gldt_stake_canister_id, &None);
    let rewards = user_0_positions
        .get(0)
        .unwrap()
        .claimable_rewards
        .get("GOLDAO")
        .unwrap();

    assert_eq!(rewards, &amount_1);
    let current_reward_rounds =
        get_reward_rounds(pic, Principal::anonymous(), gldt_stake_canister_id, &());
    assert_eq!(current_reward_rounds.len(), 1);

    process_oldest_reward_round(pic, controller, gldt_stake_canister_id, &()).unwrap();
    let user_0_positions = get_active_user_positions(pic, user_0, gldt_stake_canister_id, &None);
    let rewards = user_0_positions
        .get(0)
        .unwrap()
        .claimable_rewards
        .get("GOLDAO")
        .unwrap();

    assert_eq!(rewards, &(amount_1.clone() + amount_2.clone()));
    let current_reward_rounds =
        get_reward_rounds(pic, Principal::anonymous(), gldt_stake_canister_id, &());
    assert_eq!(current_reward_rounds.len(), 0);

    // try to process when there are no rounds left
    let res = process_oldest_reward_round(pic, controller, gldt_stake_canister_id, &());
    assert_matches!(res, Err(_));

    // check the total_allocated_rewards
    let total_rewards_allocated =
        get_total_allocated_rewards(pic, Principal::anonymous(), gldt_stake_canister_id, &());

    assert_eq!(
        total_rewards_allocated.get("GOLDAO").unwrap(),
        &Nat::from(amount_1.clone() + amount_2.clone())
    )
}
