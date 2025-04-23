use std::{collections::HashMap, time::Duration};

use assert_matches::assert_matches;
use candid::{Nat, Principal};
use canister_time::MINUTE_IN_MS;
use gldt_stake_api_canister::create_stake_position;
use gldt_stake_common::{
    ledgers::GLDT_TX_FEE,
    stake_position::{StakePositionId, StakePositionResponse},
};
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::PocketIc;
use sns_governance_canister::types::Neuron;

use crate::{
    client::{
        gldt_stake::{create_stake_position, unstake_early},
        icrc1::client::{balance_of, transfer},
        icrc1_icrc2_token::icrc2_approve,
    },
    utils::{random_principal, tick_n_blocks},
};

pub fn create_stake_position_util(
    pic: &PocketIc,
    controller: Principal,
    token_ledgers: &HashMap<String, Principal>,
    gldt_stake_canister_id: Principal,
    stake_amount: u128,
) -> (Principal, StakePositionResponse) {
    let gldt_ledger_id = token_ledgers.get("gldt_ledger_canister_id").unwrap();

    let user_1 = random_principal();

    let _ = transfer(
        pic,
        controller.clone(),
        gldt_ledger_id.clone(),
        None,
        Account {
            owner: user_1,
            subaccount: None,
        },
        stake_amount + (GLDT_TX_FEE as u128) * 2,
    );

    let balance = balance_of(
        pic,
        gldt_ledger_id.clone(),
        Account {
            owner: user_1,
            subaccount: None,
        },
    );

    assert_eq!(balance, Nat::from(stake_amount + (GLDT_TX_FEE as u128) * 2));

    // approve the required minimum stake amount
    let res = icrc2_approve(
        pic,
        user_1,
        gldt_ledger_id.clone(),
        &(icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: gldt_stake_canister_id.clone(),
                subaccount: None,
            },
            amount: Nat::from(stake_amount + GLDT_TX_FEE as u128),
            expected_allowance: Some(Nat::from(0u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        }),
    );
    assert_eq!(matches!(res, icrc2_approve::Response::Ok(_)), true);
    tick_n_blocks(pic, 3);

    // create the stake position
    let res = create_stake_position(
        pic,
        user_1,
        gldt_stake_canister_id.clone(),
        &create_stake_position::Args {
            amount: Nat::from(stake_amount + GLDT_TX_FEE as u128),
        },
    )
    .unwrap();
    assert_eq!(res.staked, Nat::from(stake_amount));
    assert_eq!(res.age_bonus_multiplier, 1.0);

    (user_1, res)
}

pub fn create_stake_position_util_for_user(
    pic: &PocketIc,
    controller: Principal,
    token_ledgers: &HashMap<String, Principal>,
    gldt_stake_canister_id: Principal,
    stake_amount: u128,
    user_principal: Principal,
) -> (Principal, StakePositionResponse) {
    let gldt_ledger_id = token_ledgers.get("gldt_ledger_canister_id").unwrap();

    let user_1 = user_principal;

    let _ = transfer(
        pic,
        controller.clone(),
        gldt_ledger_id.clone(),
        None,
        Account {
            owner: user_1,
            subaccount: None,
        },
        stake_amount + (GLDT_TX_FEE as u128) * 2,
    );
    tick_n_blocks(pic, 1);
    let balance = balance_of(
        pic,
        gldt_ledger_id.clone(),
        Account {
            owner: user_1,
            subaccount: None,
        },
    );

    // approve the required minimum stake amount
    let res = icrc2_approve(
        pic,
        user_1,
        gldt_ledger_id.clone(),
        &(icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: gldt_stake_canister_id.clone(),
                subaccount: None,
            },
            amount: Nat::from(stake_amount + GLDT_TX_FEE as u128),
            expected_allowance: Some(Nat::from(0u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        }),
    );
    assert_eq!(matches!(res, icrc2_approve::Response::Ok(_)), true);
    tick_n_blocks(pic, 3);

    // create the stake position
    let res = create_stake_position(
        pic,
        user_1,
        gldt_stake_canister_id.clone(),
        &create_stake_position::Args {
            amount: Nat::from(stake_amount + GLDT_TX_FEE as u128),
        },
    )
    .unwrap();
    assert_eq!(res.staked, Nat::from(stake_amount));
    assert_eq!(res.age_bonus_multiplier, 1.0);
    tick_n_blocks(pic, 1);

    (user_1, res)
}

pub fn create_multiple_early_unstaked_positions(
    pic: &PocketIc,
    controller: Principal,
    token_ledgers: &HashMap<String, Principal>,
    gldt_stake_canister_id: Principal,
    num_users: usize,
    num_positions_per_user: usize,
) -> Vec<(Principal, Vec<StakePositionId>)> {
    // create 10 stake positions for 10 different users with a total of 100_000_000_000 staked
    let mut return_data: Vec<(Principal, Vec<StakePositionId>)> = vec![];

    for user_index in 0..num_users {
        let user_principal = random_principal();
        println!("user prin : {user_principal}");
        let mut positions: Vec<StakePositionId> = vec![];
        for position_index in 0..num_positions_per_user {
            let (user, stake_position) = create_stake_position_util_for_user(
                pic,
                controller,
                &token_ledgers,
                gldt_stake_canister_id,
                1_000_000_000u128,
                user_principal,
            );
            let position_id = stake_position.id;

            let res = unstake_early(pic, user_principal, gldt_stake_canister_id, &position_id);
            tick_n_blocks(pic, 5);
            assert_matches!(res, Ok(_));
            positions.push(position_id);
            pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        }
        return_data.push((user_principal, positions));
    }
    return_data
}

// adds 10,000 neuron rewards of each token type
pub fn add_rewards_to_neurons(
    pic: &PocketIc,
    neuron_data: HashMap<usize, Neuron>,
    controller: Principal,
    token_ledgers: &HashMap<String, Principal>,
    gld_rewards_canister_id: Principal,
    gldt_stake_canister_id: Principal,
    ledger_fees: HashMap<String, Nat>,
) {
    let goldao_ledger = token_ledgers
        .get("goldao_ledger_canister_id")
        .unwrap()
        .clone();
    let goldao_tx_fee: u128 = ledger_fees
        .get("GOLDAO")
        .unwrap()
        .clone()
        .0
        .try_into()
        .unwrap();
    let icp_ledger = token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let icp_tx_fee: u128 = ledger_fees
        .get("ICP")
        .unwrap()
        .clone()
        .0
        .try_into()
        .unwrap();
    let ogy_ledger = token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let ogy_tx_fee: u128 = ledger_fees
        .get("OGY")
        .unwrap()
        .clone()
        .0
        .try_into()
        .unwrap();
    // add rewards to neuron so that the staking backend can claim them and allocate rewards.
    neuron_data.into_iter().for_each(|(_, neuron)| {
        let neuron_id = neuron.id.unwrap();
        let neuron_account = Account {
            owner: gld_rewards_canister_id,
            subaccount: Some(neuron_id.clone().into()),
        };
        assert_eq!(
            neuron.permissions.get(0).unwrap().principal,
            Some(gldt_stake_canister_id)
        );

        transfer(
            pic,
            controller,
            goldao_ledger,
            None,
            neuron_account,
            500_000_000_000 + goldao_tx_fee, // 5,000 GOLDAO + 1 x fee GL
        )
        .unwrap();
        assert_eq!(
            balance_of(pic, goldao_ledger, neuron_account),
            Nat::from(500_000_000_000u128 + goldao_tx_fee)
        );

        transfer(
            pic,
            controller,
            ogy_ledger,
            None,
            neuron_account,
            500_000_000_000 + ogy_tx_fee, // 5,000 GOLDAO + 1 x fee GL
        )
        .unwrap();
        assert_eq!(
            balance_of(pic, ogy_ledger, neuron_account),
            Nat::from(500_000_000_000u128 + ogy_tx_fee)
        );

        transfer(
            pic,
            controller,
            icp_ledger,
            None,
            neuron_account,
            500_000_000_000 + icp_tx_fee, // 5,000 GOLDAO + 1 x fee GL
        )
        .unwrap();
        assert_eq!(
            balance_of(pic, icp_ledger, neuron_account),
            Nat::from(500_000_000_000u128 + icp_tx_fee)
        );
    });

    // 2 neurons each with 5000 of each token meaning a total reward of 10,000 tokens per reward token type
    // 10k ICP,
    // 10k ogy
    // 10K GOLDAO
}
