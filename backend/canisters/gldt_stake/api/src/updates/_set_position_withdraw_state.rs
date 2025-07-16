use candid::{CandidType, Principal};
use gldt_stake_common::stake_position_event::WithdrawState;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub principal: Principal,
    pub state: WithdrawState,
}

pub type Response = Result<(), String>;
