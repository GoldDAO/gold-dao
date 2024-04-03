use ic_cdk_macros::query;
use sns_governance_canister::types::NeuronId;

#[query(hidden = true)]
fn convert_neuron_disp(neuron_id: NeuronId) -> String {
    neuron_id.to_string()
}
