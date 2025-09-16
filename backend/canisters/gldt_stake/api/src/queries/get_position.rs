use candid::Principal;
use gldt_stake_common::stake_position_response::StakePositionResponse;

pub type Args = Principal;
pub type Response = Option<StakePositionResponse>;
