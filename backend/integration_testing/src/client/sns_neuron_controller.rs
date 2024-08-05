use crate::generate_pocket_update_call;
use sns_neuron_controller_api_canister::manage_sns_neuron;
use sns_neuron_controller_api_canister::stake_sns_neuron;

// Queries

// Updates
generate_pocket_update_call!(manage_sns_neuron);
generate_pocket_update_call!(stake_sns_neuron);
