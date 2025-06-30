use crate::client::icrc1::client::balance_of;
use crate::client::sns_neuron_controller::stake_sns_neuron;
use crate::{
    client::icrc1::client::transfer, sns_neuron_controller_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};
use candid::{CandidType, Deserialize};
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use sns_neuron_controller_api_canister::neuron_type::NeuronType;
use std::time::Duration;

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_stake_sns_neuron_happy_path() {
    let test_env = default_test_setup();

    let wtn_ledger_canister_id = test_env.wtn_sns_test_env.ledger_id;
    let sns_neuron_controller_id = test_env.sns_neuron_controller_id;

    let staked_amount = 3_000_000_000_000 as u64;

    transfer(
        &test_env.get_pic(),
        test_env.wtn_sns_test_env.governance_id,
        wtn_ledger_canister_id,
        None,
        sns_neuron_controller_id,
        (staked_amount + 1_000_000 as u64).into(), // We add fee here in order to cover it while staking
    )
    .unwrap();

    test_env.get_pic().advance_time(Duration::from_secs(100));
    tick_n_blocks(&test_env.get_pic(), 10);

    let balance = balance_of(
        &test_env.get_pic(),
        wtn_ledger_canister_id,
        sns_neuron_controller_id,
    );
    println!("balance: {}", balance);

    test_env
        .get_pic()
        .advance_time(Duration::from_secs(24 * 60 * 60));

    let response = stake_sns_neuron(
        &test_env.get_pic(),
        test_env.wtn_sns_test_env.controller,
        sns_neuron_controller_id,
        &sns_neuron_controller_api_canister::stake_sns_neuron::Args {
            amount: staked_amount,
            neuron_type: NeuronType::WTN,
            add_disolve_delay: Some(10000000),
        },
    );

    let _neuron_id = match response {
        sns_neuron_controller_api_canister::stake_sns_neuron::Response::Success(neuron_id) => {
            NeuronId { id: neuron_id }
        }
        sns_neuron_controller_api_canister::stake_sns_neuron::Response::InternalError(error) => {
            panic!("error: {}", error);
        }
    };

    test_env
        .get_pic()
        .advance_time(Duration::from_secs(24 * 60 * 60));
    tick_n_blocks(&test_env.get_pic(), 50);

    let neurons = crate::client::sns_governance::list_neurons(
        &test_env.get_pic(),
        test_env.controller,
        test_env.wtn_sns_test_env.governance_id,
        &sns_governance_canister::types::ListNeurons {
            limit: 100,
            start_page_at: None,
            of_principal: Some(test_env.sns_neuron_controller_id),
        },
    );
    println!("neurons {:?}", neurons);
    assert_eq!(neurons.neurons.len(), 1);

    let neurons = crate::client::sns_neuron_controller::list_neurons(
        &test_env.get_pic(),
        test_env.controller,
        test_env.sns_neuron_controller_id,
        &(),
    );

    println!("neurons: {:?}", neurons);
    assert_eq!(neurons.neurons.wtn_neurons.len(), 1);
}
