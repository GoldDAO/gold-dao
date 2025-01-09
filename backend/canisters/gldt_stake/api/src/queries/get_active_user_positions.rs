use candid::Principal;
use gldt_stake_common::stake_position::StakePositionResponse;

pub type Args = Option<Principal>;
pub type Response = Vec<StakePositionResponse>;
