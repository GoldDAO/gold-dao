use candid::{CandidType, Principal};
use gldt_stake_common::stake_position::StakePositionResponse;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub user: Principal,
    pub start: usize,
    pub limit: usize,
}
pub type Response = Vec<StakePositionResponse>;
