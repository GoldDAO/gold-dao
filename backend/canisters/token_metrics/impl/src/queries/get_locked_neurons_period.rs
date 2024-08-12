use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_locked_neurons_period::Response as GetLockedNeuronsPeriodResponse;

use crate::state::read_state;

#[query]
fn get_locked_neurons_period() -> GetLockedNeuronsPeriodResponse {
    read_state(|state| state.data.locked_neurons_amount.clone())
}
