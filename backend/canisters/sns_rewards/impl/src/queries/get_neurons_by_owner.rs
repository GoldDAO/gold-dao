use candid::Principal;
use ic_cdk::query;
pub use sns_rewards_api_canister::get_neurons_by_owner::Response as GetNeuronByOwnerResponse;
use utils::env::Environment;

use crate::state::read_state;

#[query]
async fn get_neurons_by_owner() -> GetNeuronByOwnerResponse {
    let caller = read_state(|s| s.env.caller());
    get_neurons_by_owner_impl(caller)
}

pub fn get_neurons_by_owner_impl(caller: Principal) -> GetNeuronByOwnerResponse {
    read_state(|s| s.data.neuron_owners.get_neuron_ids_by_owner(caller))
}
