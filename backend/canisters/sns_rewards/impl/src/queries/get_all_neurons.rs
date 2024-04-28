use ic_cdk_macros::query;
pub use sns_rewards_api_canister::get_all_neurons::Response as GetAllNeuronsResponse;

use crate::state::read_state;

#[query(hidden = true)]
fn get_all_neurons() -> GetAllNeuronsResponse {
    read_state(|state| state.data.neuron_maturity.len())
}
