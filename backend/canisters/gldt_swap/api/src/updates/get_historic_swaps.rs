use candid::CandidType;
use serde::{Deserialize, Serialize};

use gldt_swap_common::swap::{SwapId, SwapInfo};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub page: usize,
    pub limit: usize,
}
pub type Response = Result<Vec<(SwapId, SwapInfo)>, GetHistoricSwapsError>;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum GetHistoricSwapsError {
    LimitTooLarge(String),
}
