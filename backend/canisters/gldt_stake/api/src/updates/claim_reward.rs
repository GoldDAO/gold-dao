use candid::CandidType;
use gldt_stake_common::{
    reward_tokens::TokenSymbol,
    stake_position::{ClaimRewardErrors, StakePositionId, StakePositionResponse},
};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub id: StakePositionId,
    pub token: TokenSymbol,
}

pub type Response = Result<StakePositionResponse, ClaimRewardErrors>;
