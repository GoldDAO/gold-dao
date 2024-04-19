use std::{ borrow::BorrowMut, thread, time::Duration };

use candid::{ CandidType, Deserialize, Principal };
use pocket_ic::PocketIc;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;

use crate::{
    client::rewards::{ get_all_neurons, get_neuron_by_id, sync_neurons_manual_trigger },
    setup::{
        setup::{ init, TestEnv },
        setup_sns::{ generate_neuron_data_for_week, setup_sns_by_week },
    },
};

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn test_synchronise_neurons_happy_path() {
    let env = init();
    let TestEnv { mut pic, controller, token_ledgers, mut sns, rewards } = env;
    let sns_gov_id = sns.sns_gov_id.clone();

    let all_neurons = get_all_neurons(&pic, Principal::anonymous(), rewards, &());
    assert_eq!(all_neurons as usize, sns.neuron_test_data.len());

    let single_neuron = get_neuron_by_id(
        &pic,
        Principal::anonymous(),
        rewards,
        &NeuronId::new("146ed81314556807536d74005f4121b8769bba1992fce6b90c2949e855d04208").unwrap()
    ).unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 0);

    // week 2
    sns.setup_week(&mut pic, controller, 2, sns_gov_id);
    pic.advance_time(Duration::from_secs(60 * 60 * 25)); // 25 hours
    // sync_neurons_manual_trigger(&mut pic, Principal::anonymous(), rewards, &());
    pic.tick();

    let single_neuron = get_neuron_by_id(
        &pic,
        Principal::anonymous(),
        rewards,
        &NeuronId::new("146ed81314556807536d74005f4121b8769bba1992fce6b90c2949e855d04208").unwrap()
    ).unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 100_000);

    // week 3
    sns.setup_week(&mut pic, controller, 3, sns_gov_id);
    pic.advance_time(Duration::from_secs(60 * 60 * 24)); // 25 hours
    // sync_neurons_manual_trigger(&mut pic, Principal::anonymous(), rewards, &());
    pic.tick();
    pic.advance_time(Duration::from_secs(20));

    let single_neuron = get_neuron_by_id(
        &pic,
        Principal::anonymous(),
        rewards,
        &NeuronId::new("146ed81314556807536d74005f4121b8769bba1992fce6b90c2949e855d04208").unwrap()
    ).unwrap();
    assert_eq!(single_neuron.accumulated_maturity, 200_000);
}
