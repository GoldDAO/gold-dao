use candid::Nat;
use candid::{CandidType, Deserialize};
use gldt_stake_api_canister::create_stake_position;
use gldt_stake_common::ledgers::GLDT_TX_FEE;
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;

use crate::client::gldt_stake::{create_stake_position, get_active_user_positions};
use crate::client::icrc1_icrc2_token::icrc2_approve;
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::utils::random_principal;
use crate::{
    client::icrc1::client::{balance_of, transfer},
    gldt_stake_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn create_stake_position_works() {
    let mut test_env = default_test_setup();
    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        ..
    } = test_env;

    let gldt_ledger_id = token_ledgers.get("gldt_ledger_canister_id").unwrap();

    let user_1 = random_principal();

    let _ = transfer(
        pic,
        controller,
        gldt_ledger_id.clone(),
        None,
        Account {
            owner: user_1,
            subaccount: None,
        },
        2_000_000_000u128,
    );

    let balance = balance_of(
        pic,
        gldt_ledger_id.clone(),
        Account {
            owner: user_1,
            subaccount: None,
        },
    );

    assert_eq!(balance, Nat::from(2_000_000_000u64));

    // approve the required minimum stake amount
    let res = icrc2_approve(
        pic,
        user_1,
        gldt_ledger_id.clone(),
        &(icrc2_approve::Args {
            from_subaccount: None,
            spender: Account {
                owner: gldt_stake_canister_id,
                subaccount: None,
            },
            amount: Nat::from(1_000_000_000u128 + GLDT_TX_FEE as u128),
            expected_allowance: Some(Nat::from(0u64)),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        }),
    );
    println!("{res:?}");
    assert_eq!(matches!(res, icrc2_approve::Response::Ok(_)), true);
    tick_n_blocks(pic, 2);

    // create the stake position
    let res = create_stake_position(
        pic,
        user_1,
        gldt_stake_canister_id,
        &create_stake_position::Args {
            amount: Nat::from(1_000_000_000u128 + GLDT_TX_FEE as u128),
        },
    )
    .unwrap();
    assert_eq!(res.staked, Nat::from(1_000_000_000u64));
    assert_eq!(res.age_bonus_multiplier, 1.0);

    // get user stake positions
    let positions = get_active_user_positions(pic, user_1, gldt_stake_canister_id, &(None));
    assert_eq!(positions.len(), 1);
}
