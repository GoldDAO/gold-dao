use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use gldt_swap_common::swap::{SwapId, SwapInfo};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub page: usize,
    pub limit: usize,
    pub user: Principal,
}
pub type Response = Result<Vec<(SwapId, SwapInfo)>, GetHistoricSwapsByUserError>;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum GetHistoricSwapsByUserError {
    LimitTooLarge(String),
    LimitTooSmall(String),
    QueryCanisterError(String),
}
