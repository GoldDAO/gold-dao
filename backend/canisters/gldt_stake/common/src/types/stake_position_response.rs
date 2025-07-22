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

        // Adjust claimable rewards by deducting fees and filtering out tokens below the fee
        let claimable_rewards = position
            .claimable_rewards
            .iter()
            .filter_map(|(symbol, amount)| {
                let token_info = symbol.get_token_info();
                if amount > &token_info.fee {
                    let net_amount = amount.clone() - token_info.fee;
                    Some((symbol.clone(), net_amount))
                } else {
                    None
                }
            })
            .collect();

        StakePositionResponse {
            owned_by: position.owned_by,
            staked: position.staked.clone(),
            created_at: position.created_at,
            claimable_rewards,
            dissolve_delay: position.dissolve_delay,
            age_bonus_multiplier,
            weighted_stake,
            instant_dissolve_fee,
            dissolve_events: position.dissolve_events.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stake_position::StakePosition;
    use crate::stake_position_event::ClaimRewardStatus;
    use crate::stake_position_event::WithdrawState;
    use candid::Nat;
    use std::collections::HashMap;
    use std::time::Duration;
    use types::TokenSymbol;

    fn dummy_stake_position_with_rewards(rewards: Vec<(TokenSymbol, u128)>) -> StakePosition {
        let claimable_rewards: HashMap<TokenSymbol, Nat> = rewards
            .into_iter()
            .map(|(sym, val)| (sym, Nat::from(val)))
            .collect();

        StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000_000_000u128),
            created_at: 0,
            dissolve_delay: Duration::from_secs(0),
            claimable_rewards,
            dissolve_events: vec![],
            age_bonus_timestamp: 0,
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
        }
    }

    #[test]
    fn test_claimable_rewards_filters_and_deducts_fees() {
        // Using real token symbols with known fees
        let rewards = vec![
            (TokenSymbol::ICP, 20_000),      // fee: 10_000 → included (10_000)
            (TokenSymbol::OGY, 150_000),     // fee: 200_000 → excluded
            (TokenSymbol::GOLDAO, 250_000),  // fee: 100_000 → included (150_000)
            (TokenSymbol::WTN, 1_000_000),   // fee: 1_000_000 → excluded (equal to fee)
            (TokenSymbol::GLDT, 20_000_000), // fee: 10_000_000 → included (10_000_000)
        ];

        let position = dummy_stake_position_with_rewards(rewards);
        let response: StakePositionResponse = (&position, 0).into();

        let expected: HashMap<TokenSymbol, Nat> = vec![
            (TokenSymbol::ICP, Nat::from(10_000u64)),
            (TokenSymbol::GOLDAO, Nat::from(150_000u64)),
            (TokenSymbol::GLDT, Nat::from(10_000_000u64)),
        ]
        .into_iter()
        .collect();

        assert_eq!(response.claimable_rewards, expected);
    }
}
