use std::collections::HashMap;
use std::collections::VecDeque;

use candid::Nat;
use gldt_stake_common::reward_round::RewardRound;
use gldt_stake_common::reward_round::RewardRoundStatus;
use gldt_stake_common::reward_tokens::TokenSymbol;
use serde::{Deserialize, Serialize};
use tracing::debug;
use types::TimestampMillis;

#[derive(Serialize, Deserialize, Clone)]
pub struct RewardSystem {
    // rounds that are due to be processed
    pub rounds: VecDeque<RewardRound>,
    // all the previous rewards added together when a round has been processed. useful for APY calculations
    pub reward_history: HashMap<TokenSymbol, Nat>,
}

impl Default for RewardSystem {
    fn default() -> Self {
        let mut reward_history = HashMap::new();
        reward_history.insert("ICP".to_string(), Nat::from(0u64));
        reward_history.insert("OGY".to_string(), Nat::from(0u64));
        reward_history.insert("GLDGov".to_string(), Nat::from(0u64));
        Self {
            rounds: VecDeque::default(),
            reward_history,
        }
    }
}
// front is the oldest
// back is the newest
impl RewardSystem {
    /// Add a new reward allocation round to the system
    pub fn add_reward_round(
        &mut self,
        rewards: Nat,
        token_symbol: TokenSymbol,
        timestamp: TimestampMillis,
    ) -> RewardRound {
        let round = RewardRound::new(rewards, token_symbol, timestamp);
        self.rounds.push_back(round.clone());
        round
    }

    /// Get a reference to the latest round (the front of the queue) without removing it
    pub fn peek_oldest_round(&self) -> Option<&RewardRound> {
        self.rounds.front()
    }

    /// Get a mutable reference to the latest round (the front of the queue) without removing it
    pub fn peek_oldest_round_mut(&mut self) -> Option<&mut RewardRound> {
        self.rounds.front_mut()
    }

    pub fn remove_oldest_round(&mut self) {
        self.rounds.pop_front();
    }

    pub fn peek_newest_round(&self) -> Option<&RewardRound> {
        self.rounds.back()
    }

    pub fn peek_newest_round_mut(&mut self) -> Option<&mut RewardRound> {
        self.rounds.back_mut()
    }

    pub fn set_oldest_round_status(&mut self, new_status: RewardRoundStatus) {
        match self.rounds.front_mut() {
            Some(round) => round.status = new_status,
            None => {
                debug!("No rounds to execute on")
            }
        }
    }

    pub fn get_all_reward_rounds(&self) -> Vec<RewardRound> {
        self.rounds.iter().cloned().collect()
    }

    pub fn add_to_reward_history(&mut self, token_symbol: &TokenSymbol, rewards: Nat) {
        self.reward_history
            .entry(token_symbol.clone())
            .and_modify(|current_reward| *current_reward += rewards);
    }
}
