use candid::{CandidType, Nat, Principal};
use gldt_stake_common::stake_position::{StakePosition, StakePositionId};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub start: Nat,
    pub limit: usize,
    pub user_principal: Option<Principal>,
}
pub type Response = Vec<(StakePositionId, StakePosition)>;
