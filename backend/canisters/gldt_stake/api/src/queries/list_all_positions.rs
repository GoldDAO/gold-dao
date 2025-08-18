use candid::CandidType;
use candid::Principal;
use gldt_stake_common::stake_position_response::StakePositionResponse;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub of_principal: Option<Principal>,
    pub limit: u64,
    pub skip: u64,
}
pub type Response = Vec<StakePositionResponse>;
