use std::collections::HashMap;
use std::collections::VecDeque;

use candid::Nat;
use gldt_stake_common::reward_round::RewardRound;
use gldt_stake_common::reward_round::RewardRoundStatus;
use gldt_stake_common::reward_tokens::TokenSymbol;
use serde::{Deserialize, Serialize};
use tracing::debug;
use types::TimestampMillis;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RewardSystem {
    // rounds that are due to be processed
    pub rounds: VecDeque<RewardRound>,
    // all the previous rewards added together when a round has been processed. useful for APY calculations
    pub reward_history: HashMap<TokenSymbol, Nat>,
    // tracks weekly rewards that have been allocated so that we may calculate a weekly reward APY. this value is set to 0 each week in the calculate_weekly_apy job once a weekly APY has been calcualted
    pub weekly_allocated_rewards: HashMap<TimestampMillis, HashMap<TokenSymbol, Nat>>, // weekly reward history - keeps track of the total rewards for each week that have been allocated for each token
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
            .and_modify(|current_reward| *current_reward += rewards.clone())
            .or_insert(rewards.clone());
    }

    pub fn add_reward(&mut self, timestamp: TimestampMillis, token: TokenSymbol, amount: Nat) {
        self.weekly_allocated_rewards
            .entry(timestamp) // Get or insert the inner HashMap
            .or_insert_with(HashMap::new)
            .entry(token) // Get or insert the Nat value
            .and_modify(|existing| *existing += amount.clone())
            .or_insert(amount);
    }
}
