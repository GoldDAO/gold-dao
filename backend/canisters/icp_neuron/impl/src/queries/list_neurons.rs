use candid::CandidType;
use ic_cdk::query;

use crate::state::{read_state, NeuronList};

#[derive(CandidType)]
pub struct ListNeuronsResponse {
    neurons: NeuronList,
}

#[query]
fn list_neurons() -> ListNeuronsResponse {
    read_state(|s| ListNeuronsResponse {
        neurons: s.data.get_neuron_list(),
    })
}
