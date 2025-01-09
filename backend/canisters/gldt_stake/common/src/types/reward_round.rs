use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};
use types::TimestampMillis;

use super::reward_tokens::TokenSymbol;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct RewardRound {
    created_at: TimestampMillis,
    pub rewards: Nat,
    pub token_symbol: TokenSymbol,
    pub status: RewardRoundStatus,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum RewardRoundStatus {
    RewardsClaimed,
    AllocationInProgress,
    RewardsAllocated,
}

impl RewardRound {
    pub fn new(rewards: Nat, token_symbol: TokenSymbol, timestamp: TimestampMillis) -> Self {
        Self {
            created_at: timestamp,
            rewards: rewards,
            token_symbol: token_symbol,
            status: RewardRoundStatus::RewardsClaimed,
        }
    }

    pub fn get_rewards(&self) -> &Nat {
        &self.rewards
    }

    pub fn get_token_symbol(&self) -> &TokenSymbol {
        &self.token_symbol
    }

    pub fn get_status(&self) -> &RewardRoundStatus {
        &self.status
    }

    pub fn get_round_timestamp(&self) -> TimestampMillis {
        self.created_at
    }
}
