use gldt_stake_common::stake_position::StakePosition;
use std::collections::HashMap;

pub type Args = ();
pub type Response = HashMap<candid::Principal, StakePosition>;
