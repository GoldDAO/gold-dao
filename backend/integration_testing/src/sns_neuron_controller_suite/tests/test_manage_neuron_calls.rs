use crate::client::sns_neuron_controller;
use crate::sns_neuron_controller_suite::setup::default_test_setup;
use candid::{CandidType, Deserialize};
use serde::Serialize;
use sns_governance_canister::types::manage_neuron::AddNeuronPermissions;
use sns_governance_canister::types::manage_neuron::Command::AddNeuronPermissions as AddNeuronPermissionsCommand;
use sns_governance_canister::types::NeuronId;
use sns_neuron_controller_api_canister::neuron_type::NeuronType;

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
// TODO: extend this test too
fn test_process_neurons_happy_path() {
    let mut test_env = default_test_setup();

    // NOTE: call for managing the neuron
    sns_neuron_controller::manage_sns_neuron(
        &mut test_env.pic,
        test_env.controller,
        test_env.sns_neuron_controller_id,
        &sns_neuron_controller_api_canister::manage_sns_neuron::Args {
            neuron_type: NeuronType::Ogy,
            neuron_id: vec![0],
            command: AddNeuronPermissionsCommand(AddNeuronPermissions {
                principal_id: None,
                permissions_to_add: None,
            }),
        },
    );
    test_env.pic.tick();
}
