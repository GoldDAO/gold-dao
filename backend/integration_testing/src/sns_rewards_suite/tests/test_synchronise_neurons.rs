use std::time::Duration;
use candid::{ CandidType, Deserialize };
use canister_time::DAY_IN_MS;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;

use crate::{
    client::rewards::{ get_all_neurons, get_neuron_by_id },
    sns_rewards_suite::setup::default_test_setup,
    utils::{ random_principal, tick_n_blocks },
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_synchronise_neurons_happy_path() {
    let mut test_env = default_test_setup();

    let all_neurons = get_all_neurons(
        &test_env.pic,
        random_principal(),
        test_env.rewards_canister_id.clone(),
        &()
    );
    assert_eq!(all_neurons as usize, test_env.neuron_data.len());

    let neuron_id_1 = test_env.neuron_data.get(&1usize).unwrap().clone().id.unwrap();
    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        random_principal(),
        test_env.rewards_canister_id.clone(),
        &neuron_id_1
    ).unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 0);

    // day 1
    test_env.simulate_neuron_voting(2);
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS)); // 25 hours
    tick_n_blocks(&test_env.pic, 50);

    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        random_principal(),
        test_env.rewards_canister_id.clone(),
        &neuron_id_1
    ).unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 100_000);
    tick_n_blocks(&test_env.pic, 2);

    // day 2
    test_env.simulate_neuron_voting(3);
    test_env.pic.advance_time(Duration::from_millis(DAY_IN_MS)); // 25 hours
    tick_n_blocks(&test_env.pic, 50);

    let single_neuron = get_neuron_by_id(
        &test_env.pic,
        random_principal(),
        test_env.rewards_canister_id.clone(),
        &neuron_id_1
    ).unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 200_000);
}
