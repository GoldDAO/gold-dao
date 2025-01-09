use candid::CandidType;
use gldt_stake_common::{stake_position::StakePositionId, stake_position_event::UnstakeState};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub id: StakePositionId,
    pub state: UnstakeState,
}

pub type Response = Result<(), String>;
