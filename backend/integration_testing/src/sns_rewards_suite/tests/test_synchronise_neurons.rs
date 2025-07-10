use canister_time::{DAY_IN_MS, HOUR_IN_MS};
use std::time::{Duration, SystemTime};

use crate::{
    client::rewards::{get_all_neurons, get_neuron_by_id},
    sns_rewards_suite::setup::default_test_setup,
    utils::{random_principal, tick_n_blocks},
};

#[test]
fn test_synchronise_neurons_happy_path() {
    let test_env = default_test_setup();
    let pic = test_env.pic.borrow();
    pic.set_time((SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(1718776800000)).into()); // Wednesday Jun 19, 2024, 6:00:00 AM

    pic.advance_time(Duration::from_millis(HOUR_IN_MS * 3));
    tick_n_blocks(&pic, 10);

    let all_neurons = get_all_neurons(
        &pic,
        random_principal(),
        test_env.rewards_canister_id.clone(),
        &(),
    );
    assert_eq!(all_neurons as usize, test_env.neuron_data.len());

    let neuron_id_1 = test_env
        .neuron_data
        .get(&1usize)
        .unwrap()
        .clone()
        .id
        .unwrap();
    let single_neuron = get_neuron_by_id(
        &pic,
        random_principal(),
        test_env.rewards_canister_id.clone(),
        &neuron_id_1,
    )
    .unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 0);

    // day 1
    test_env.simulate_neuron_voting(2);
    pic.advance_time(Duration::from_millis(DAY_IN_MS)); // 25 hours
    tick_n_blocks(&pic, 50);

    let single_neuron = get_neuron_by_id(
        &pic,
        random_principal(),
        test_env.rewards_canister_id.clone(),
        &neuron_id_1,
    )
    .unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 100_000);
    tick_n_blocks(&pic, 2);

    // day 2
    test_env.simulate_neuron_voting(3);
    pic.advance_time(Duration::from_millis(DAY_IN_MS)); // 25 hours
    tick_n_blocks(&pic, 50);

    let single_neuron = get_neuron_by_id(
        &pic,
        random_principal(),
        test_env.rewards_canister_id.clone(),
        &neuron_id_1,
    )
    .unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 200_000);
}
