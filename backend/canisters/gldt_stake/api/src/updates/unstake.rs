use gldt_stake_common::stake_position::{
    StakePositionId, StakePositionResponse, UnstakeRequestErrors,
};

pub type Args = StakePositionId;

pub type Response = Result<StakePositionResponse, UnstakeRequestErrors>;
