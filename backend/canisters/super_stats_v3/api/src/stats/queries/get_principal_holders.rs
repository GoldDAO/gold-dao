use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::custom_types::HolderBalanceResponse;

#[derive(CandidType, Deserialize, Serialize)]
pub struct GetHoldersArgs {
    pub offset: u64,
    pub limit: u64,
}

pub type Args = GetHoldersArgs;
pub type Response = Vec<HolderBalanceResponse>;
