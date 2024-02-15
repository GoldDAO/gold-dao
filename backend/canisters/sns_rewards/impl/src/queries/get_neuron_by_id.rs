use ic_cdk_macros::query;
use sns_governance_canister::types::NeuronId;
use types::NeuronInfo;
use crate::state::read_state;

#[query]
fn get_neuron_by_id(id: NeuronId) -> Option<NeuronInfo> {
    read_state(|state| { state.neuron_maturity.get(&id).cloned() })
}
