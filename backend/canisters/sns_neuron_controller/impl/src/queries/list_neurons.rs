use ic_cdk::query;

use crate::state::read_state;
pub use sns_neuron_controller_api_canister::list_neurons::Args as ListNeuronsArgs;
pub use sns_neuron_controller_api_canister::list_neurons::Response as ListNeuronsResponse;

#[query]
fn list_neurons() -> ListNeuronsResponse {
    read_state(|s| ListNeuronsResponse {
        neurons: s.data.neuron_managers.get_neurons(),
    })
}
