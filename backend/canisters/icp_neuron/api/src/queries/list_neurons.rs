use candid::CandidType;
use icp_neuron_common::neuron_list::NeuronList;

#[derive(CandidType)]
pub struct ListNeuronsResponse {
    pub neurons: NeuronList,
}
