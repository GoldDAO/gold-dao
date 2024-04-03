use candid::Principal;
use ic_cdk::{ caller, query };
use sns_governance_canister::types::NeuronId;

use crate::state::read_state;

#[query]
async fn get_neurons_by_owner() -> Option<Vec<NeuronId>> {
    get_neurons_by_owner_impl(caller())
}

pub fn get_neurons_by_owner_impl(caller: Principal) -> Option<Vec<NeuronId>> {
    read_state(|s| s.data.neuron_owners.get_neuron_ids_by_owner(caller))
}
