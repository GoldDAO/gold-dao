use candid::Principal;
use ic_cdk_macros::query;

use crate::state::read_state;

#[query]
fn get_all_neuron_owners() -> Vec<Principal> {
    read_state(|state| state.data.principal_neurons.keys().cloned().collect())
}
