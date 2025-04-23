use std::collections::HashMap;

use crate::memory::{get_stake_positions_memory, get_weekly_apy_memory, VM};
use candid::{Nat, Principal};
use canister_time::{timestamp_millis, WEEK_IN_MS};
use gldt_stake_common::{
    reward_tokens::{RewardTypes, TokenSymbol},
    stake_position::{StakePosition, StakePositionId},
};
use ic_stable_structures::BTreeMap as StableBTreeMap;
use serde::{Deserialize, Serialize};
use types::TimestampMillis;

#[derive(Serialize, Deserialize)]
pub struct StakeSystem {
    /// stable memory storage of stake positions
    #[serde(skip, default = "init_stake_system_memory")]
    stakes: StableBTreeMap<StakePositionId, StakePosition, VM>,
    /// id of the latest index
    pub current_stake_index: StakePositionId,
    /// total gldt staked
    pub total_staked: Nat,
    /// a cached total weighted stake
    pub cached_total_weighted_stake: Nat,
    /// number of total active stake positions
    pub total_stake_positions: usize,
    /// reward types for distribution
    pub reward_types: RewardTypes,
    /// available to transfer to fee account
    pub pending_fee_transfer_amount: Nat,
    /// the date time that the canister went live set in init - used for APY calculations - calculating an average of weekly rewards based on the number of weeks that has passed
    pub genesis_datetime: TimestampMillis,
    /// usd price of reward tokens + gldt - used for APY calculations
    pub token_usd_values: HashMap<TokenSymbol, f64>,
    /// weekly APY computations available for querying
    #[serde(skip, default = "init_weekly_apy_history")]
    pub weekly_apy_history: StableBTreeMap<TimestampMillis, f64, VM>,
    // tracks the weekly timestamp for calculating weekly APY
    pub weekly_apy_timestamp: TimestampMillis,
    // tracks the weekly staked GLDT
    pub weekly_weighted_staked_gldt: HashMap<TimestampMillis, Nat>,
}

fn init_stake_system_memory() -> StableBTreeMap<StakePositionId, StakePosition, VM> {
    let memory = get_stake_positions_memory();
    StableBTreeMap::init(memory)
}
fn init_weekly_apy_history() -> StableBTreeMap<TimestampMillis, f64, VM> {
    let memory = get_weekly_apy_memory();
    StableBTreeMap::init(memory)
}

impl Default for StakeSystem {
    fn default() -> Self {
        Self {
            stakes: init_stake_system_memory(),
            current_stake_index: StakePositionId::from(0u64),
            total_staked: Nat::from(0u64),
            total_stake_positions: 0usize,
            cached_total_weighted_stake: Nat::from(0u64),
            reward_types: HashMap::new(),
            pending_fee_transfer_amount: Nat::from(0u64),
            genesis_datetime: timestamp_millis(),
            token_usd_values: HashMap::new(),
            weekly_apy_history: init_weekly_apy_history(),
            weekly_apy_timestamp: timestamp_millis(),
            weekly_weighted_staked_gldt: HashMap::new(),
        }
    }
}

impl StakeSystem {
    pub fn add_stake_position(
        &mut self,
        stake_amount: Nat,
        user: Principal,
    ) -> (StakePositionId, StakePosition) {
        let new_position = StakePosition::new(user, stake_amount);
        let id = self.current_stake_index;
        self.stakes
            .insert(self.current_stake_index, new_position.clone());
        self.total_staked += new_position.staked.clone();
        self.total_stake_positions += 1;
        self.current_stake_index += 1;
        (id, new_position)
    }

    pub fn remove_stake_position(&mut self, stake_id: StakePositionId) -> Option<StakePosition> {
        self.stakes.remove(&stake_id)
    }

    pub fn get_stake_position(&self, stake_id: StakePositionId) -> Option<StakePosition> {
        self.stakes.get(&stake_id)
    }

    pub fn get_stake_positions_by_user(
        &self,
        user: &Principal,
    ) -> Vec<(StakePositionId, StakePosition)> {
        self.stakes
            .iter()
            .filter(|(_, stake_position)| &stake_position.owned_by == user)
            .collect()
    }

    pub fn get_reward_eligible_stake_positions(&self) -> Vec<(StakePositionId, StakePosition)> {
        self.stakes
            .iter()
            .filter(|(_, stake_position)| stake_position.eligible_for_reward_allocation())
            .collect()
    }

    pub fn count_user_stake_positions(&self, user: &Principal) -> usize {
        self.stakes
            .iter()
            .filter(|(_, stake_position)| &stake_position.owned_by == user)
            .collect::<Vec<_>>()
            .len()
    }

    pub fn calculate_total_weighted_stake_for_timestamp(&mut self, time: TimestampMillis) -> Nat {
        let total = self
            .stakes
            .iter()
            .fold(Nat::from(0u64), |acc, (_id, stake_position)| {
                if stake_position.eligible_for_reward_allocation() {
                    let bonus_multiplier = stake_position.calculate_age_bonus_multiplier(time);
                    let weighted_stake = stake_position.calculate_weighted_stake(bonus_multiplier);
                    acc + weighted_stake
                } else {
                    acc
                }
            });
        self.cached_total_weighted_stake = total.clone();
        total
    }

    pub fn update_stake_position(
        &mut self,
        id: &StakePositionId,
        updated_position: StakePosition,
    ) -> Option<StakePosition> {
        self.stakes.insert(*id, updated_position)
    }

    pub fn set_token_usd_values(&mut self, values: HashMap<TokenSymbol, f64>) {
        self.token_usd_values = values;
    }

    pub fn bump_weekly_timestamp(&mut self) {
        self.weekly_apy_timestamp += WEEK_IN_MS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::{Nat, Principal};

    use crate::state::{init_state, mutate_state, read_state, RuntimeState};

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    #[test]
    pub fn test_add_stake_position_basic() {
        init_runtime_state();

        let user_a = Principal::anonymous();
        let stake_amount = Nat::from(1000u64);
        let (position_id, _) = mutate_state(|s| {
            s.data
                .stake_system
                .add_stake_position(stake_amount.clone(), user_a)
        });

        let position = read_state(|s| s.data.stake_system.get_stake_position(position_id));
        assert_eq!(position.is_some(), true);
        assert_eq!(read_state(|s| s.data.stake_system.total_stake_positions), 1);
        assert_eq!(
            read_state(|s| s.data.stake_system.total_staked.clone()),
            stake_amount
        );
    }
}
