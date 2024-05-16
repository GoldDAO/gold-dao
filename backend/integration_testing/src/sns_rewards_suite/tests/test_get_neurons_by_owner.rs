use candid::{ CandidType, Deserialize, Principal };
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::add_neuron_ownership::Response as AddNeuronOwnerShipResponse;

use crate::{
    client::{ icrc1::client::transfer, rewards::{ add_neuron_ownership, get_neurons_by_owner } },
    sns_rewards_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_reward_claim_happy_path() {
    let mut test_env = default_test_setup();

    let icp_ledger_id = test_env.token_ledgers.get("icp_ledger_canister_id").unwrap().clone();
    let rewards_canister_id = test_env.rewards_canister_id;

    let user_1 = test_env.users.get(0).unwrap().clone();
    let neuron_1 = test_env.neuron_data.get(&0usize).unwrap().clone();
    let neuron_id_1 = test_env.neuron_data.get(&0usize).unwrap().clone().id.unwrap();
    assert!(neuron_1.permissions.get(1).unwrap().principal == Some(user_1)); // double check the data correct ( user_1's hotkey is on the first neuron's permissions list )

    // ********************************
    // 1. simulate distribution - add reward to neuron
    // ********************************
    let neuron_account_1 = Account {
        owner: rewards_canister_id,
        subaccount: Some(neuron_id_1.clone().into()),
    };
    transfer(
        &mut test_env.pic,
        test_env.sns_gov_canister_id,
        icp_ledger_id,
        None,
        neuron_account_1,
        (100_000_000_00u64).into()
    ).unwrap();
    tick_n_blocks(&test_env.pic, 10);

    // ********************************
    // 2. add ownership
    // ********************************
    let res = add_neuron_ownership(
        &mut test_env.pic,
        user_1,
        rewards_canister_id,
        &neuron_id_1.clone()
    );
    println!("{res:?}");
    tick_n_blocks(&test_env.pic, 10);
    match res {
        AddNeuronOwnerShipResponse::Ok(n_id) => assert_eq!(n_id, neuron_id_1),
        _ => {}
    }
    tick_n_blocks(&test_env.pic, 10);

    // passing no arguments should work because the caller is user_1
    let res = get_neurons_by_owner(&test_env.pic, user_1, rewards_canister_id, &None);
    assert_eq!(res.unwrap().len(), 1);

    // passing the user principal as an argument should behave the same with an anonymous caller
    let res = get_neurons_by_owner(
        &test_env.pic,
        Principal::anonymous(),
        rewards_canister_id,
        &Some(user_1)
    );
    assert_eq!(res.unwrap().len(), 1);
}
