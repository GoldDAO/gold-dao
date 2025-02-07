use ic_cdk::query;
pub use icp_neuron_api_canister::list_neurons::ListNeuronsResponse;

use crate::state::read_state;

#[query]
fn list_neurons() -> ListNeuronsResponse {
    read_state(|s| ListNeuronsResponse {
        neurons: s.data.get_neuron_list(),
    })
}
