use crate::client::icrc1::client::balance_of;
use crate::client::sns_neuron_controller::list_neurons;
use crate::{
    sns_neuron_controller_suite::setup::test_setup_with_predefined_wtn_neurons,
    utils::tick_n_blocks,
};
use candid::{CandidType, Deserialize};
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use std::time::Duration;
#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}
use candid::Nat;

#[test]
fn test_disburse_maturity() {
    let test_env = test_setup_with_predefined_wtn_neurons();

    println!("test_env: {:#?}", test_env);

    test_env.get_pic().advance_time(Duration::from_secs(100));
    tick_n_blocks(&test_env.get_pic(), 10);

    test_env
        .get_pic()
        .advance_time(Duration::from_secs(24 * 60 * 60));

    let neurons = list_neurons(
        &test_env.get_pic(),
        test_env.wtn_sns_test_env.governance_id,
        test_env.sns_neuron_controller_id,
        &(),
    );

    println!("neurons: {:?}", neurons);

    test_env.get_pic().advance_time(Duration::from_secs(100));
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
    let neuron_while_disbursement = neurons.neurons.first().unwrap();

    test_env
        .get_pic()
        .advance_time(Duration::from_secs(9 * 24 * 60 * 60));
    tick_n_blocks(&test_env.get_pic(), 100);

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

    let neuron_after_disbursement = neurons.neurons.first().unwrap();

    assert!(!neuron_while_disbursement
        .disburse_maturity_in_progress
        .is_empty());
    assert!(neuron_after_disbursement
        .disburse_maturity_in_progress
        .is_empty());

    let rewards_account_balance = balance_of(
        &test_env.get_pic(),
        test_env.wtn_sns_test_env.ledger_id,
        Account {
            owner: test_env.gld_rewards_canister_id.clone(),
            subaccount: None,
        },
    );
    println!("rewards_account_balance: {:?}", rewards_account_balance);
    assert_eq!(Nat::from(1000000_u64), rewards_account_balance);
}
