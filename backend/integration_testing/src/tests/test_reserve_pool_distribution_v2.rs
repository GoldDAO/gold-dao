use std::{ borrow::BorrowMut, thread, time::Duration };

use candid::{ CandidType, Deserialize, Nat, Principal };
use canister_time::DAY_IN_MS;
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::PocketIc;
use serde::Serialize;
use serde_bytes::ByteBuf;
use sns_governance_canister::types::NeuronId;
use sns_rewards::consts::{ RESERVE_POOL_SUB_ACCOUNT, REWARD_POOL_SUB_ACCOUNT };

use crate::{
    client::{
        icrc1::client::{ balance_of, transfer },
        rewards::{ get_all_neurons, get_neuron_by_id, http_request, sync_neurons_manual_trigger },
    },
    setup::setup::{ default_test_setup, init, TestEnv },
    utils::tick_n_blocks,
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_reserve_pool_distribution_happy_path() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let ogy_ledger_id = test_env.token_ledgers.get("ogy_ledger_canister_id").unwrap().clone();
    let gldgov_ledger_id = test_env.token_ledgers.get("gldgov_ledger_canister_id").unwrap().clone();
    let controller = test_env.controller;
    let sns_gov_id = test_env.sns_gov_canister_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let user_2 = test_env.users.get(1).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();

    let reward_pool = Account {
        owner: rewards_canister_id,
        subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
    };

    let reserve_pool_account = Account {
        owner: rewards_canister_id,
        subaccount: Some(RESERVE_POOL_SUB_ACCOUNT),
    };

    // setup always gives a starting amount to reward pools
    let gldgov_reward_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reward_pool);
    assert_eq!(gldgov_reward_pool_balance, Nat::from(100_000_000_000u64));

    // TRIGGER - reserve_pool_distribution cron job
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // reward pool should be the same since there was nothing in the reserve pool to transfer
    let gldgov_reward_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reward_pool);
    assert_eq!(gldgov_reward_pool_balance, Nat::from(100_000_000_000u64));

    // transfer some gldgov to the reserve pool
    transfer(
        &mut test_env.pic,
        controller,
        gldgov_ledger_id,
        None,
        reserve_pool_account,
        (100_000_000_000u64).into()
    ).unwrap();

    // TRIGGER - reserve_pool_distribution cron job
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS));
    tick_n_blocks(&test_env.pic, 100);

    // reward pool should now have double minus a fee
    let gldgov_reward_pool_balance = balance_of(&test_env.pic, gldgov_ledger_id, reward_pool);
    tick_n_blocks(&test_env.pic, 2);
    let expected_balance_reward_pool = Nat::from(100_000_000_000u64 + 100_000_000u64);
    assert_eq!(expected_balance_reward_pool, gldgov_reward_pool_balance);
}
