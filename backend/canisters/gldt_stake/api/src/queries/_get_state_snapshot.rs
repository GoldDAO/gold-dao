use candid::{CandidType, Nat};
use gldt_stake_common::stake_position::StakePosition;
use serde::Deserialize;
use serde::Serialize;

pub type Args = ();
pub type Response = StateSnapshot;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct StateSnapshot {
    pub position: Option<StakePosition>,
    pub total_staked: Nat,
}
