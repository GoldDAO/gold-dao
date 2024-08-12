pub use candid::CandidType;
use serde::{ Deserialize, Serialize };
use types::TimestampMillis;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub swap_id: u128,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SwapStatus),
    NotFound,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapStatus {
    Init,
    Deposit,
    Transfer,
    WIthdraw,
    Complete,
}
