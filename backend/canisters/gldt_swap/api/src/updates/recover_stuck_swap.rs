use candid::CandidType;
use serde::{ Deserialize, Serialize };

use gldt_swap_common::swap::SwapId;

pub type Args = SwapId;
pub type Response = Result<SwapId, RecoverSwapError>;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum RecoverSwapError {
    NoSwapExists,
    NoSwapDetails,
    SwapIsNotStuck,
    InvalidForwardSwapType(String),
    InProgress,
}
