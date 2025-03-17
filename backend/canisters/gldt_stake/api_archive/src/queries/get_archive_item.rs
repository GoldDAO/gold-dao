use gldt_stake_common::stake_position::{StakePosition, StakePositionId};

pub type Args = StakePositionId;
pub type Response = Option<(StakePositionId, StakePosition)>;
