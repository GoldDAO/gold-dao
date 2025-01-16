use candid::{CandidType, Nat};
use gldt_stake_common::stake_position::{AddStakePositionErrors, StakePositionResponse};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub amount: Nat,
}

pub type Response = Result<StakePositionResponse, AddStakePositionErrors>;
