use crate::generate_pocket_query_call;
use crate::generate_pocket_update_call;
use sns_neuron_controller_api_canister::list_neurons;
use sns_neuron_controller_api_canister::manage_sns_neuron;
use sns_neuron_controller_api_canister::stake_sns_neuron;

// Queries
generate_pocket_query_call!(list_neurons);

// Updates
generate_pocket_update_call!(manage_sns_neuron);
generate_pocket_update_call!(stake_sns_neuron);
