use std::{ borrow::BorrowMut, thread, time::Duration };

use candid::{ CandidType, Deserialize, Nat, Principal };
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::PocketIc;
use serde::Serialize;
use serde_bytes::ByteBuf;
use sns_governance_canister::types::NeuronId;
use sns_rewards::consts::{ RESERVE_POOL_SUB_ACCOUNT, REWARD_POOL_SUB_ACCOUNT };

use crate::{
    client::{
        icrc1::happy_path::{ balance_of, transfer },
        rewards::{ get_all_neurons, get_neuron_by_id, http_request, sync_neurons_manual_trigger },
    },
    setup::{ setup::{ init, TestEnv }, sns::{ generate_neuron_data_for_week, setup_sns_by_week } },
    utils::{ decode_http_bytes, tick_n_blocks },
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_reserve_pool_distribution_happy_path() {
    let env = init();
    let TestEnv { mut pic, controller, token_ledgers, mut sns, rewards } = env;
    let sns_gov_id = sns.sns_gov_id.clone();

    let reward_pool = Account {
        owner: rewards,
        subaccount: Some(REWARD_POOL_SUB_ACCOUNT),
    };

    let icp_reward_pool_balance = balance_of(&pic, token_ledgers.gldgov_ledger_id, reward_pool);
    assert_eq!(icp_reward_pool_balance, Nat::from(100_000_000_000u64));

    pic.advance_time(Duration::from_secs(60 * 60 * 25)); // 1 day + 1 hour
    tick_n_blocks(&pic, 20);

    // reward pool should be the same since there was nothing in the reserve pool to transfer
    let icp_reward_pool_balance = balance_of(&pic, token_ledgers.gldgov_ledger_id, reward_pool);
    assert_eq!(icp_reward_pool_balance, Nat::from(100_000_000_000u64));

    // transfer some gldgov to the reserve pool
    let reserve_pool_account = Account {
        owner: rewards,
        subaccount: Some(RESERVE_POOL_SUB_ACCOUNT),
    };
    transfer(
        &mut pic,
        controller,
        token_ledgers.gldgov_ledger_id,
        reserve_pool_account,
        (100_000_000_00u64).into()
    ).unwrap();
    pic.tick();
    pic.advance_time(Duration::from_secs(60 * 60 * 24)); // 1 day
    tick_n_blocks(&pic, 200);
    // reward pool should now have double minus a fee
    let icp_reward_pool_balance = balance_of(&pic, token_ledgers.gldgov_ledger_id, reward_pool);
    let res = http_request(
        &pic,
        Principal::anonymous(),
        rewards,
        &(types::HttpRequest {
            method: "GET".to_string(),
            url: "/logs".to_string(),
            headers: vec![],
            body: ByteBuf::new(),
        })
    );
    println!("{}", decode_http_bytes(res.body.into_vec()));
    let expected_balance_reward_pool = Nat::from(100_000_000_000u64 + 100_000_000u64);
    assert_eq!(icp_reward_pool_balance, expected_balance_reward_pool);
}
