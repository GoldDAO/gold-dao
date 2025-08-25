// use crate::memory::{get_daily_apy_memory, VM};
use candid::{Nat, Principal};
use canister_time::{timestamp_millis, DAY_IN_MS};
use gldt_stake_common::stake_position::StakePosition;
use ic_stable_structures::BTreeMap as StableBTreeMap;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::collections::HashMap;
use tracing::error;
use types::TimestampMillis;
use types::TokenSymbol;

#[derive(Serialize, Deserialize)]
pub struct StakeSystem {
    pub stakes: HashMap<Principal, StakePosition>,
    pub total_staked: Nat,
    pub cached_total_weighted_stake: Nat,
    pub stake_positions_quantity_limit: usize,
    pub reward_types: BTreeSet<TokenSymbol>,
    // available to transfer to fee account
    pub pending_fee_transfer_amount: Nat,
    // the date time that the canister went live set in init - used for APY calculations - calculating an average of weekly rewards based on the number of weeks that has passed
    pub genesis_datetime: TimestampMillis,
    // usd price of reward tokens + gldt - used for APY calculations
    pub token_usd_values: HashMap<TokenSymbol, f64>,

    // #[serde(skip, default = "init_daily_apy_history")]
    // pub daily_apy_history: StableBTreeMap<TimestampMillis, f64, VM>,
    pub cached_daily_timestamp: TimestampMillis,
    // pub daily_weighted_staked_gldt: HashMap<TimestampMillis, Nat>,
    // pub daily_staked_gldt: HashMap<TimestampMillis, Nat>,
}

// pub fn init_daily_apy_history() -> StableBTreeMap<TimestampMillis, f64, VM> {
//     let memory = get_daily_apy_memory();
//     StableBTreeMap::init(memory)
// }

impl Default for StakeSystem {
    fn default() -> Self {
        let now = timestamp_millis();
        Self {
            stakes: HashMap::new(),
            total_staked: Nat::from(0u64),
            cached_total_weighted_stake: Nat::from(0u64),
            stake_positions_quantity_limit: 100_000,
            reward_types: BTreeSet::new(),
            pending_fee_transfer_amount: Nat::from(0u64),
            genesis_datetime: now,
            token_usd_values: HashMap::new(),
            // daily_apy_history: init_daily_apy_history(),
            cached_daily_timestamp: now,
            // daily_weighted_staked_gldt: HashMap::new(),
            // daily_staked_gldt: HashMap::new(),
        }
    }
}

impl StakeSystem {
    pub fn active_stake_positions(&self) -> usize {
        self.stakes.len()
    }

    pub fn can_add_new_stake_position(&self) -> bool {
        self.stakes.len() < self.stake_positions_quantity_limit
    }

    pub fn remove_all_empty_stake_positions(&mut self) -> Vec<candid::Principal> {
        let users_to_remove: Vec<Principal> = self
            .stakes
            .iter()
            .filter(|(_user, pos)| {
                !pos.has_rewards() && pos.staked == 0u128 && pos.dissolve_events.is_empty()
            })
            .map(|(user, _)| *user)
            .collect();

        for user in &users_to_remove {
            match self.remove_stake_position(user) {
                Ok(_) => {}
                Err(err) => error!(
                    "Failed to remove stake position for user {}: {}",
                    user.to_text(),
                    err
                ),
            }
        }

        users_to_remove
    }

    pub fn remove_stake_position(&mut self, user: &Principal) -> Result<StakePosition, String> {
        let removed = self.stakes.get(user).ok_or_else(|| {
            let msg = format!("No stake position found for user {}", user.to_text());
            msg
        })?;

        if removed.staked != 0u128 {
            let msg = format!(
                "Cannot remove stake position for user {}: staked amount is not zero ({}).",
                user.to_text(),
                removed.staked
            );
            return Err(msg);
        }

        if removed.dissolve_events.iter().any(|e| !e.completed) {
            let msg = format!(
            "Cannot remove stake position for user {}: there are unfinished dissolve events: {:?}.",
            user.to_text(),
            removed.dissolve_events
        );
            return Err(msg);
        }

        let removed = self
            .stakes
            .remove(user)
            .expect("Stake existed before, but missing during remove");

        Ok(removed)
    }

    /// Returns the stake position associated with the given user if it exists.
    pub fn get_stake_position(&self, user: &Principal) -> Option<StakePosition> {
        self.stakes.get(user).cloned()
    }

    pub fn get_reward_eligible_stake_positions(&self) -> Vec<(Principal, StakePosition)> {
        self.stakes
            .iter()
            .filter(|(_, pos)| pos.can_allocate_reward().is_ok())
            .map(|(user, pos)| (*user, pos.clone()))
            .collect()
    }

    pub fn upsert_stake_position(
        &mut self,
        user: Principal,
        updated_position: StakePosition,
    ) -> Option<StakePosition> {
        self.stakes.insert(user, updated_position)
    }

    pub fn set_token_usd_values(&mut self, values: HashMap<TokenSymbol, f64>) {
        self.token_usd_values = values;
    }

    pub fn bump_daily_timestamp(&mut self) {
        self.cached_daily_timestamp += DAY_IN_MS;
    }
}
