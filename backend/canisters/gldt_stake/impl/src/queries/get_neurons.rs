use canister_tracing_macros::trace;

pub use gldt_stake_api_canister::queries::get_neurons::{
    Args as GetNeuronsRequest, Response as GetNeuronsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
async fn get_neurons(_args: GetNeuronsRequest) -> GetNeuronsResponse {
    read_state(|s| s.data.neuron_system.get_neurons())
}
