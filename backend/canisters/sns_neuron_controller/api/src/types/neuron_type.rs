use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum NeuronType {
    Ogy,
    Wtn,
}
