use candid::CandidType;
use serde::{Deserialize, Serialize};

use gldt_swap_common::swap::SwapIndex;

pub type Args = SwapIndex;
pub type Response = Result<SwapIndex, RecoverSwapError>;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum RecoverSwapError {
    NoSwapExists,
    NoSwapDetails,
    SwapIsNotStuck,
    InvalidForwardSwapType(String),
    InProgress,
}
