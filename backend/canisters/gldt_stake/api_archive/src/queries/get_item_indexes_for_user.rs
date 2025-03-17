use candid::Principal;
use gldt_stake_common::stake_position::StakePositionId;

pub type Args = Principal;
pub type Response = Vec<StakePositionId>;
