use crate::types::list_neurons_type::NeuronList;
use candid::CandidType;
use serde::Deserialize;

pub type Args = ();
#[derive(CandidType, Deserialize, Debug)]
pub struct Response {
    pub neurons: NeuronList,
}
