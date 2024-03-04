use ic_cdk_macros::query;
use sns_governance_canister::types::NeuronId;
use types::{ NeuronInfo, TimestampMillis };

use crate::state::read_state;

pub type MaturityHistoryResponse = Vec<(TimestampMillis, NeuronInfo)>;

#[query]
fn get_maturity_history_of_neuron(
    neuron_id: NeuronId,
    size: Option<usize>
) -> MaturityHistoryResponse {
    read_state(|state| {
        state.data.maturity_history.get_maturity_history(neuron_id, size.unwrap_or(100))
    })
}

// no real use for this, mainly for testing. Remove later
#[query]
fn get_n_history(size: Option<usize>) -> Vec<((NeuronId, TimestampMillis), NeuronInfo)> {
    read_state(|state| { state.data.maturity_history.get(size.unwrap_or(100)) })
}
