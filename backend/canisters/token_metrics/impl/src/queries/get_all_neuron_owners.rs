use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_all_neuron_owners::Response as GetAllNeuronOwnersResponse;

use crate::state::read_state;

#[query]
fn get_all_neuron_owners() -> GetAllNeuronOwnersResponse {
    read_state(|state| state.data.principal_neurons.keys().cloned().collect())
}
