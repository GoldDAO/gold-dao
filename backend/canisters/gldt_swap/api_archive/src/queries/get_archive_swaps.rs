use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use gldt_swap_common::swap::{SwapId, SwapIndex, SwapInfo};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub start: SwapIndex,
    pub limit: usize,
    pub user_principal: Option<Principal>,
}
pub type Response = Vec<(SwapId, SwapInfo)>;
