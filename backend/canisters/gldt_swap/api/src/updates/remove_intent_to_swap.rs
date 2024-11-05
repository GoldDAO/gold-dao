use candid::CandidType;
use serde::{Deserialize, Serialize};

use gldt_swap_common::swap::SwapId;

pub type Args = SwapId;

pub type Response = Result<(), RemoveIntentToSwapError>;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum RemoveIntentToSwapError {
    SwapNotFound,
    InProgress,
    InvalidSwapType(String),
    InvalidUser,
}
