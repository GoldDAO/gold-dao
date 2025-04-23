use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};
use types::TimestampMillis;

use super::{reward_tokens::TokenSymbol, stake_position::StakePosition};

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct RewardRound {
    pub created_at: TimestampMillis,
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
            rewards,
            token_symbol,
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

    pub fn calculate_total_weighted_stake(&self, stake_positions: &[(u64, StakePosition)]) -> Nat {
        stake_positions
            .iter()
            .fold(Nat::from(0u64), |acc, (_, position)| {
                let age_bonus_multiplier = position.calculate_age_bonus_multiplier(self.created_at);
                let weighted_stake = position.calculate_weighted_stake(age_bonus_multiplier);
                acc + weighted_stake
            })
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Duration};

    use candid::Principal;
    use canister_time::{timestamp_millis, DAY_IN_MS};

    use crate::{
        stake_position::{DissolveState, GLDT_STAKE_DISSOLVE_DELAY_MS},
        stake_position_event::{ClaimRewardStatus, UnstakeState},
    };

    use super::*;

    #[test]
    fn test_calculate_total_weighted_stake() {
        let now: TimestampMillis = timestamp_millis();
        let one_year_in_ms = (365.25 * DAY_IN_MS as f64) as u64; // Account for leap years

        // 5 stake positions
        // all created 1 year ago ( 2x bonus modifier )
        // expect in total we have 5 x ( 2 x 1000 ) = 10_000
        let position_1 = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - one_year_in_ms,
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        };
        let position_2 = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - one_year_in_ms,
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        };
        let position_3 = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - one_year_in_ms,
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        };
        let position_4 = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - one_year_in_ms,
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        };
        let position_5 = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - one_year_in_ms,
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        };

        let positions = vec![
            (0u64, position_1),
            (1u64, position_2),
            (2u64, position_3),
            (3u64, position_4),
            (4u64, position_5),
        ];

        let round = RewardRound::new(Nat::from(0u64), "OGY".to_string(), now);
        let result = round.calculate_total_weighted_stake(&positions);
        assert_eq!(result, Nat::from(10_000u64));
    }
}
