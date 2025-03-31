use crate::neuron_type::NeuronType;
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub amount: u64,
    pub neuron_type: NeuronType,
    pub add_disolve_delay: Option<u32>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(Vec<u8>),
    InternalError(String),
}
