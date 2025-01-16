use gldt_stake_common::stake_position::{
    StakePositionId, StakePositionResponse, StartDissolvingErrors,
};

pub type Args = StakePositionId;

pub type Response = Result<StakePositionResponse, StartDissolvingErrors>;
