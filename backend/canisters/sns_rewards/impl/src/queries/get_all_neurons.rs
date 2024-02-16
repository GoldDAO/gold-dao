use ic_cdk_macros::query;

use crate::state::read_state;

#[query]
fn get_all_neurons() -> usize {
    read_state(|state| state.data.neuron_maturity.len())
}
