use candid::CandidType;
use candid::Nat;
use candid::Principal;
use canister_time::timestamp_millis;
use gldt_stake_common::accounts::ALLOCATED_REWARDS_POOL;
use icrc_ledger_canister_c2c_client::icrc1_balance_of;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::TimestampMillis;
use types::TokenSymbol;

pub const ALLOCATE_REWARDS_THRESHOLD: u64 = 100_000_000_u64;

#[allow(async_fn_in_trait)]
pub trait AllocatedRewards {
    const SUBACCOUNT: Subaccount = ALLOCATED_REWARDS_POOL;

    fn account() -> Account {
        Account {
            owner: ic_cdk::api::canister_self(),
            subaccount: Some(Self::SUBACCOUNT),
        }
    }

    async fn balance(&self, token_ledger: Principal) -> Result<Nat, String> {
        match icrc1_balance_of(token_ledger, Self::account()).await {
            Ok(balance) => Ok(balance),
            Err(e) => Err(format!(
                "AllocatedRewardsPool : fetch_pool_balance error: {e:?}"
            )),
        }
    }
}

impl AllocatedRewards for AllocatedRewardsPool {}

#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct AllocatedRewardsPool {
    pub state: AllocatedRewardsState,
    pub last_allocation_time: TimestampMillis,
    pub reward_history: HashMap<TokenSymbol, Nat>, // all the previous rewards added together when a transfer from processing pool has been processed. useful for APY calculations
    pub daily_allocated_rewards: HashMap<TimestampMillis, HashMap<TokenSymbol, Nat>>, // daily reward history - keeps track of the total rewards for each week that have been allocated for each token
}

impl AllocatedRewardsPool {
    pub fn new_allocated() -> Self {
        Self {
            state: AllocatedRewardsState::default(),
            last_allocation_time: 0,
            reward_history: HashMap::default(),
            daily_allocated_rewards: HashMap::default(),
        }
    }

    pub fn add_reward(&mut self, timestamp: TimestampMillis, token: TokenSymbol, amount: Nat) {
        self.daily_allocated_rewards
            .entry(timestamp) // Get or insert the inner HashMap
            .or_default()
            .entry(token) // Get or insert the Nat value
            .and_modify(|existing| *existing += amount.clone())
            .or_insert(amount);
    }

    pub fn add_to_reward_history(&mut self, token_symbol: &TokenSymbol, rewards: Nat) {
        self.reward_history
            .entry(*token_symbol)
            .and_modify(|current_reward| *current_reward += rewards.clone())
            .or_insert(rewards.clone());
    }

    pub fn current_state(&self) -> &AllocatedRewardsState {
        &self.state
    }

    pub fn is_awaiting(&self) -> bool {
        matches!(self.state, AllocatedRewardsState::Awaiting)
    }

    pub fn transition_to_allocating(&mut self) {
        self.state = match self.state {
            AllocatedRewardsState::Awaiting => AllocatedRewardsState::Allocating,
            _ => AllocatedRewardsState::Error(format!(
                "Invalid transition from {:?} to Allocating",
                self.state
            )),
        };
    }

    pub fn transition_to_awaiting(&mut self) {
        self.state = AllocatedRewardsState::Awaiting;
    }

    pub fn transition_to_error(&mut self, msg: String) {
        self.state = AllocatedRewardsState::Error(msg);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, CandidType)]
pub enum AllocatedRewardsState {
    #[default]
    Awaiting,
    Allocating,
    Error(String),
}

use gldt_stake_common::stake_position::StakePosition;
pub fn calculate_total_weighted_stake(stake_positions: &[(Principal, StakePosition)]) -> Nat {
    let now = timestamp_millis();
    stake_positions
        .iter()
        .fold(Nat::from(0u64), |acc, (_, position)| {
            let age_bonus_multiplier = position.calculate_age_bonus_multiplier(now);
            let weighted_stake = position.calculate_weighted_stake(age_bonus_multiplier);
            acc + weighted_stake
        })
}
