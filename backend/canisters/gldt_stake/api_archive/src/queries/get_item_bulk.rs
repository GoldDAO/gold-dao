use gldt_stake_common::stake_position::{StakePosition, StakePositionId};

pub type Args = Vec<StakePositionId>;
pub type Response = Vec<(StakePositionId, StakePosition)>;
