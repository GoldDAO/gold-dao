use crate::stake_position::StakePosition;
use crate::stake_position_event::DissolveStakeEvent;
use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use types::{TimestampMillis, TokenSymbol};

#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub struct StakePositionResponse {
    pub owned_by: Principal,
    pub staked: Nat,
    pub created_at: TimestampMillis,
    pub age_bonus_multiplier: f64,
    pub weighted_stake: Nat,
    pub claimable_rewards: HashMap<TokenSymbol, Nat>,
    pub dissolve_delay: Duration,
    pub instant_dissolve_fee: Nat,
    pub dissolve_events: Vec<DissolveStakeEvent>,
}

impl From<(&StakePosition, TimestampMillis)> for StakePositionResponse {
    fn from((position, timestamp): (&StakePosition, TimestampMillis)) -> Self {
        StakePositionResponse::from_parts(position, timestamp)
    }
}

impl From<(StakePosition, TimestampMillis)> for StakePositionResponse {
    fn from((position, timestamp): (StakePosition, TimestampMillis)) -> Self {
        StakePositionResponse::from_parts(&position, timestamp)
    }
}

impl From<(&mut StakePosition, TimestampMillis)> for StakePositionResponse {
    fn from((position, timestamp): (&mut StakePosition, TimestampMillis)) -> Self {
        StakePositionResponse::from_parts(&*position, timestamp)
    }
}

impl StakePositionResponse {
    pub fn from_parts(position: &StakePosition, timestamp: TimestampMillis) -> Self {
        let age_bonus_multiplier = position.calculate_age_bonus_multiplier(timestamp);
        let weighted_stake = position.calculate_weighted_stake(age_bonus_multiplier);
        let instant_dissolve_fee =
            position.calculate_dissolve_instantly_fee(position.staked.clone());

        StakePositionResponse {
            owned_by: position.owned_by,
            staked: position.staked.clone(),
            created_at: position.created_at,
            claimable_rewards: position.claimable_rewards.clone(),
            dissolve_delay: position.dissolve_delay,
            age_bonus_multiplier,
            weighted_stake,
            instant_dissolve_fee,
            dissolve_events: position.dissolve_events.clone(),
        }
    }
}
