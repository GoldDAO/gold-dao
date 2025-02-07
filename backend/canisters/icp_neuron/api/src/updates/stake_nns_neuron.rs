use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum StakeNnsNeuronResponse {
    Success(u64),
    InternalError(String),
}
