use gldt_stake_common::stake_position::{
    StakePositionId, StakePositionResponse, UnstakeEarlyRequestErrors,
};

pub type Args = StakePositionId;

pub type Response = Result<StakePositionResponse, UnstakeEarlyRequestErrors>;
