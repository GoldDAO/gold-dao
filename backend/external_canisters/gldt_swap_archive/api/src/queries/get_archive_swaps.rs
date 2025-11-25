use crate::swap::SwapId;
use crate::swap::SwapIndex;
use crate::swap::SwapInfo;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub start: SwapIndex,
    pub limit: usize,
    pub user_principal: Option<Principal>,
}
pub type Response = Vec<(SwapId, SwapInfo)>;
