use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub amount: u64,
}

pub type Response = Result<Vec<u8>, CreateNeuronError>;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CreateNeuronError {
    TransferError(String),
    InternalError(String),
}
