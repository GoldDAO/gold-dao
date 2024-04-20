use std::time::Duration;
use candid::{ CandidType, Deserialize, Principal };
use serde::Serialize;
use sns_governance_canister::types::NeuronId;

use crate::{
    client::rewards::{ get_all_neurons, get_neuron_by_id },
    setup::{
        setup::{ default_test_setup, init, RewardsTestEnv, RewardsTestEnvBuilder, TestEnv },
        setup_sns::{ generate_neuron_data_for_week, setup_sns_by_week },
    },
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_synchronise_neurons_happy_path_v2() {
    let mut test_env = default_test_setup();

    let all_neurons = get_all_neurons(
        &test_env.pic,
        Principal::anonymous(),
        test_env.rewards_canister_id.clone(),
        &()
    );
    assert_eq!(all_neurons as usize, test_env.neuron_data.len());
    let neuron_id_1 = test_env.neuron_data.get(&1usize).unwrap().clone().id.unwrap();
    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        Principal::anonymous(),
        test_env.rewards_canister_id.clone(),
        &neuron_id_1
    ).unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 0);

    // week 2
    test_env.simulate_neuron_voting(2);
    test_env.pic.advance_time(Duration::from_secs(60 * 60 * 25)); // 25 hours
    test_env.pic.tick();

    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        Principal::anonymous(),
        test_env.rewards_canister_id.clone(),
        &neuron_id_1
    ).unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 100_000);

    // week 3
    test_env.simulate_neuron_voting(3);
    test_env.pic.advance_time(Duration::from_secs(60 * 60 * 24)); // 25 hours
    test_env.pic.tick();
    test_env.pic.advance_time(Duration::from_secs(20));

    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        Principal::anonymous(),
        test_env.rewards_canister_id.clone(),
        &neuron_id_1
    ).unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 200_000);
}
