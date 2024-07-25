use crate::neuron_type::NeuronType;
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Args {
    pub neuron_type: NeuronType,
    pub neuron_id: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(Vec<u8>),
    InternalError(String),
}
