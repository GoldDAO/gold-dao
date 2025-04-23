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
        (staked_amount + 200_000 as u64).into(), // We add fee here in order to cover it while staking
    )
    .unwrap();

    test_env.get_pic().advance_time(Duration::from_secs(100));
    tick_n_blocks(&test_env.get_pic(), 10);

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

    let neuron_id = match response {
        sns_neuron_controller_api_canister::stake_sns_neuron::Response::Success(neuron_id) => {
            NeuronId { id: neuron_id }
        }
        sns_neuron_controller_api_canister::stake_sns_neuron::Response::InternalError(error) => {
            panic!("error: {}", error);
        }
    };

    // let neurons = crate::client::sns_governance::list_neurons(
    //     &test_env.get_pic(),
    //     test_env.controller,
    //     test_env.sns_test_env.governance_id,
    //     &sns_governance_canister::types::ListNeurons {
    //         limit: 100,
    //         start_page_at: None,
    //         of_principal: Some(test_env.sns_neuron_controller_id),
    //     },
    // );
    // // println!("neurons {:?}", neurons);
    // // assert_eq!(neurons.neurons.len(), 1);

    // let neuron_response: sns_governance_canister::types::get_neuron_response::Result =
    //     crate::client::sns_governance::get_neuron(
    //         &test_env.get_pic(),
    //         test_env.controller,
    //         test_env.sns_test_env.governance_id,
    //         &sns_governance_canister::types::GetNeuron {
    //             neuron_id: Some(neuron_id.clone()),
    //         },
    //     )
    //     .result
    //     .unwrap();

    // let neuron = match neuron_response {
    //     sns_governance_canister::types::get_neuron_response::Result::Neuron(neuron) => {
    //         // NOTE: Check that the owner of the neuron is the sns_neuron_controller_id
    //         assert!(neuron.permissions.get(0).unwrap().principal == Some(sns_neuron_controller_id));
    //         neuron.clone()
    //     }
    //     sns_governance_canister::types::get_neuron_response::Result::Error(error) => {
    //         panic!("error: {:?}", error);
    //     }
    // };

    // let proposal_result = test_env
    //     .sns_test_env
    //     .submit_proposal(test_env.sns_neuron_controller_id, &neuron.id.unwrap());

    // test_env.get_pic().advance_time(Duration::from_secs(100));
    // tick_n_blocks(test_env.get_pic(), 50);

    // let vote_result = test_env.sns_test_env.vote_on_proposal(
    //     test_env.sns_neuron_controller_id,
    //     &neuron_id,
    //     1,
    //     true,
    // );

    // test_env.get_pic().advance_time(Duration::from_secs(100));
    // tick_n_blocks(&test_env.get_pic(), 50);

    // let proposals = client::sns_governance::list_proposals(
    //     &test_env.get_pic(),
    //     test_env.sns_test_env.governance_id,
    //     test_env.sns_test_env.governance_id,
    //     &sns_governance_canister::types::ListProposals {
    //         limit: 100,
    //         before_proposal: None,
    //         exclude_type: vec![],
    //         include_reward_status: vec![],
    //         include_status: vec![],
    //     },
    // );

    // println!("proposals: {:#?}", proposals);

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
