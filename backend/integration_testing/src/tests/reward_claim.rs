use std::{ borrow::BorrowMut, thread, time::Duration };

use candid::{ CandidType, Deserialize, Nat, Principal };
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::PocketIc;
use serde::Serialize;
use serde_bytes::ByteBuf;
use sns_governance_canister::types::{ neuron, NeuronId };
use sns_rewards::{
    consts::{ RESERVE_POOL_SUB_ACCOUNT, REWARD_POOL_SUB_ACCOUNT },
    types::claim_neuron_response::UserClaimErrorResponse,
};

use crate::{
    client::{
        icrc1::happy_path::{ balance_of, transfer },
        pocket::execute_update_multi_args,
        rewards::{
            add_neuron_ownership,
            claim_reward,
            get_all_neurons,
            get_neuron_by_id,
            http_request,
            sync_neurons_manual_trigger,
        },
    },
    setup::{ setup::{ init, TestEnv }, sns::{ generate_neuron_data_for_week, setup_sns_by_week } },
    utils::{ decode_http_bytes, hex_to_subaccount, tick_n_blocks },
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_reward_claim_happy_path() {
    let env = init();
    let TestEnv { mut pic, controller, token_ledgers, mut sns, rewards } = env;
    let sns_gov_id = sns.sns_gov_id.clone();

    let user_1 = sns.users.get(0).unwrap().clone();
    let user_2 = sns.users.get(1).unwrap().clone();

    // simulate a distribution by add some ICP rewards to a neuron that is owned by user_1 - see sns.rs for which neurons have users as hotkeys
    let neuron_id_1 = &NeuronId::new(
        "5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21"
    ).unwrap();
    let neuron_account_1 = Account {
        owner: rewards,
        subaccount: Some(
            hex_to_subaccount("5129ea7ec019c9a5f19b16ae3562870556b6f4cb424496f6255215a33465ea21")
        ),
    };
    transfer(
        &mut pic,
        controller,
        token_ledgers.icp_ledger_id,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&pic, 10);

    // add ownership - should return ok because user_1 has their hotkey on the neuron
    let res = add_neuron_ownership(&mut pic, user_1, rewards, &neuron_id_1.clone()).unwrap();
    tick_n_blocks(&pic, 10);
    assert_eq!(res, neuron_id_1.clone());

    // claim the reward - should return true
    let res = execute_update_multi_args::<(NeuronId, String), Result<bool, UserClaimErrorResponse>>(
        &mut pic,
        user_1,
        rewards,
        "claim_reward",
        (neuron_id_1.clone(), "ICP".to_string())
    ).unwrap();
    tick_n_blocks(&pic, 20);
    assert_eq!(res, true);

    // check the balance to verify the reward - fee exists
    let user_1_account = Account {
        owner: user_1.clone(),
        subaccount: None,
    };
    let user_1_icp_balance = balance_of(&pic, token_ledgers.icp_ledger_id, user_1_account);
    tick_n_blocks(&pic, 10);
    assert_eq!(user_1_icp_balance, Nat::from(100_000_000_00u64) - Nat::from(10_000u64));
}
