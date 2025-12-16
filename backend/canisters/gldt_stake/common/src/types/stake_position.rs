use super::stake_position_event::{ClaimRewardStatus, WithdrawState};
use super::{ledgers::GLDT_TX_FEE, reward_tokens::RewardTokens};
use crate::manage_stake_position_interface::AddStakePositionErrors;
use crate::manage_stake_position_interface::GeneralError;
use crate::manage_stake_position_interface::RemoveRewardErrors;
use crate::manage_stake_position_interface::StartDissolvingErrors;
use crate::manage_stake_position_interface::WithdrawErrors;
use crate::stake_position_event::DissolveStakeEvent;
use bity_ic_canister_time::{timestamp_millis, DAY_IN_MS};
use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use types::{TimestampMillis, TokenSymbol};
use utils::numeric::{Percentage, ScaledArithmetic};

pub const GLDT_STAKE_DISSOLVE_DELAY_MS: u64 = DAY_IN_MS * 7;
pub const GLDT_STAKE_EARLY_UNSTAKE_FEE_PERCENTAGE: f64 = 0.0;
pub const MINIMUM_STAKE_AMOUNT: u64 = 1_000_000_000; // 10 GLDT
pub const MINIMUM_STAKE_AMOUNT_WITH_FEE: u64 = MINIMUM_STAKE_AMOUNT + GLDT_TX_FEE;
pub const MAXIMUM_STAKE_AMOUNT: u64 = 100_000_000_000_000; // 1 million GLDT
pub const MAX_ACTIVE_EVENTS_PER_POSITION: usize = 5;
pub const DAYS_IN_A_YEAR: f64 = 365.25; // Account for leap years
pub const BONUS_INCREMENT: f64 = 1.0 / DAYS_IN_A_YEAR; // Daily increment

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct StakePosition {
    pub owned_by: Principal,
    pub staked: Nat,
    pub created_at: TimestampMillis,
    pub age_bonus_timestamp: TimestampMillis,
    pub claimable_rewards: RewardTokens,
    pub dissolve_delay: Duration,
    pub claim_reward_status: ClaimRewardStatus,
    pub withdraw_state: WithdrawState,
    pub dissolve_events: Vec<DissolveStakeEvent>,
}

impl StakePosition {
    pub fn try_new(
        owner: Principal,
        initial_stake_amount: Nat,
    ) -> Result<Self, AddStakePositionErrors> {
        Self::validate_stake_amount(&initial_stake_amount)?;

        let mut claimable_rewards = HashMap::new();
        claimable_rewards.insert(TokenSymbol::ICP, Nat::from(0u64));
        claimable_rewards.insert(TokenSymbol::OGY, Nat::from(0u64));
        claimable_rewards.insert(TokenSymbol::GOLDAO, Nat::from(0u64));

        let now = timestamp_millis();
        Ok(Self {
            owned_by: owner,
            staked: initial_stake_amount,
            created_at: now,
            age_bonus_timestamp: now,
            claimable_rewards,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        })
    }

    pub fn validate_stake_amount(amount: &Nat) -> Result<(), AddStakePositionErrors> {
        if *amount < MINIMUM_STAKE_AMOUNT {
            return Err(AddStakePositionErrors::InvalidStakeAmount(format!(
                "Initial stake amount {} is less than minimum {}.",
                amount, MINIMUM_STAKE_AMOUNT
            )));
        }
        if *amount > MAXIMUM_STAKE_AMOUNT {
            return Err(AddStakePositionErrors::InvalidStakeAmount(format!(
                "Initial stake amount {} exceeds maximum {}.",
                amount, MAXIMUM_STAKE_AMOUNT
            )));
        }
        Ok(())
    }

    pub fn change_stake(&mut self, amount: Nat, change: StakeChange) -> Result<(), GeneralError> {
        match change {
            StakeChange::Increase => {
                let new_stake = self.staked.clone() + amount.clone();

                if new_stake < MINIMUM_STAKE_AMOUNT {
                    return Err(GeneralError::ModifyStakeError(format!(
                    "Cannot increase stake by {}. Minimum stake is {} and currently staked is {}.",
                    amount, MINIMUM_STAKE_AMOUNT, self.staked
                )));
                }

                if new_stake > MAXIMUM_STAKE_AMOUNT {
                    return Err(GeneralError::ModifyStakeError(format!(
                    "Cannot increase stake by {}. Maximum stake is {} and currently staked is {}.",
                    amount, MAXIMUM_STAKE_AMOUNT, self.staked
                )));
                }

                self.update_age_bonus_timestamp(amount.clone());
                self.staked = new_stake;
            }

            StakeChange::Decrease(decrease_type) => {
                if amount > self.staked {
                    return Err(GeneralError::ModifyStakeError(format!(
                        "Cannot decrease stake by {}. Current stake is only {}.",
                        amount, self.staked
                    )));
                }

                let new_stake = self.staked.clone() - amount.clone();

                if matches!(decrease_type, DecreaseType::Fractional)
                    && new_stake < MINIMUM_STAKE_AMOUNT
                {
                    return Err(GeneralError::ModifyStakeError(format!(
                    "Stake would drop below minimum stake amount {} (current: {}, decrease: {}).",
                    MINIMUM_STAKE_AMOUNT, self.staked, amount
                )));
                }

                // NOTE: age bonus timestamp is not reset on decrease
                self.staked = new_stake;
            }
        }

        Ok(())
    }

    pub fn insert_event(&mut self, payload: DissolveStakeEvent) {
        self.dissolve_events.push(payload)
    }

    // NOTE: all events are considered completed
    pub fn cleanup_completed_dissolve_events(&mut self) -> Vec<DissolveStakeEvent> {
        // Partition events: those not completed go into `completed`,
        // those already completed remain in the list.
        let (completed, remaining): (Vec<_>, Vec<_>) = self
            .dissolve_events
            .drain(..)
            .partition(|event| !event.completed);

        // Keep only already-completed events
        self.dissolve_events = remaining;

        // Return all newly completed events
        completed
    }

    pub fn calculate_weighted_stake(&self, age_bonus_multiplier: f64) -> Nat {
        self.staked
            .scale_e8s_mul_f64(age_bonus_multiplier)
            .scale_e8s_down()
    }

    pub fn calculate_new_reward(
        &self,
        total_weighted_stake_pool: &Nat,
        current_timestamp: TimestampMillis,
        reward_pool_amount: &Nat,
    ) -> Nat {
        let age_bonus_multiplier = self.calculate_age_bonus_multiplier(current_timestamp);
        let position_weighted_stake = self.calculate_weighted_stake(age_bonus_multiplier);
        let percentage_scaled = position_weighted_stake.scaled_e8s_div(total_weighted_stake_pool);
        (reward_pool_amount.clone() * percentage_scaled).scale_e8s_down()
    }

    pub fn calculate_age_bonus_multiplier(&self, current_timestamp: TimestampMillis) -> f64 {
        // NOTE: dosn't depend on dissolvance anymore
        let age_in_millis = current_timestamp.saturating_sub(self.age_bonus_timestamp);

        // Convert age in milliseconds to age in days
        let age_in_days = age_in_millis as f64 / DAY_IN_MS as f64;

        // Calculate the raw bonus increment
        let raw_bonus = age_in_days * BONUS_INCREMENT;

        // If less than 1 full day has passed, the multiplier should stay at 1.0
        if age_in_days >= 1.0 {
            // If we've passed at least one full day, apply the raw bonus increment
            1.0 + raw_bonus
        } else {
            // Otherwise, return the base multiplier (1.0)
            1.0
        }
    }

    pub fn can_allocate_reward(&self) -> Result<(), GeneralError> {
        // Find the latest dissolve event (by timestamp + event_id)
        let last_dissolve = self.dissolve_events.last().map(|event| {
            let DissolveStakeEvent {
                percentage,
                completed,
                ..
            } = event;
            (*percentage, *completed)
        });

        // Check that the full stake wasn't dissolved
        if matches!(last_dissolve, Some((Percentage::MAX, false))) {
            Err(GeneralError::CannotAddReward(
                "Full stake is dissolved, cannot allocate new rewards".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn can_claim_reward(
        &self,
        token: &TokenSymbol,
        amount: &Nat,
    ) -> Result<(), RemoveRewardErrors> {
        if let Some(current_rewards) = self.claimable_rewards.get(token) {
            if amount > current_rewards || current_rewards == &Nat::from(0u64) {
                Err(RemoveRewardErrors::InsufficientBalance(format!(
                    "cant deduct a reward of {} {:?} because balance is {} {:?}",
                    amount, token, current_rewards, token
                )))
            } else {
                Ok(())
            }
        } else {
            Err(RemoveRewardErrors::RewardTokenTypeDoesNotExist(format!(
                "Token of type '{:?}' does not exist in the rewards map",
                token
            )))
        }
    }

    pub fn can_start_dissolving(&self) -> Result<(), StartDissolvingErrors> {
        if self.staked == 0u64 {
            return Err(StartDissolvingErrors::InvalidDissolveAmount(
                "Cannot start dissolving on a position with zero stake.".to_string(),
            ));
        }

        let active_dissolves = self
            .dissolve_events
            .iter()
            .filter(|event| !event.completed)
            .count();

        if active_dissolves >= MAX_ACTIVE_EVENTS_PER_POSITION {
            Err(StartDissolvingErrors::DissolvementsLimitReached(format!(
                "You already have {} active dissolve dissolve_events. Complete some before starting a new one.",
                MAX_ACTIVE_EVENTS_PER_POSITION
            )))
        } else {
            Ok(())
        }
    }

    pub fn can_dissolve_instantly(&self, percentage: Percentage) -> Result<(), WithdrawErrors> {
        self.withdraw_state.is_valid_state_to_withdraw()?;

        if percentage == 100 {
            // Must not have rewards
            if self.has_rewards() {
                return Err(WithdrawErrors::CantWithdrawWithRewardsBalance(
                    "This stake position has rewards available to claim.".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub fn can_withdraw(&self) -> Result<(), WithdrawErrors> {
        // First ensure the current withdraw state allows unstaking
        self.withdraw_state.is_valid_state_to_withdraw()?;

        // Now check if there's at least one dissolve event ready
        let now = timestamp_millis();

        // NOTE: all events are considered completed
        if self.dissolve_events.is_empty() {
            return Err(WithdrawErrors::NoValidDissolveEvents(
                "No tokens are currently eligible for unstaking.".to_string(),
            ));
        }

        Ok(())
    }

    pub fn calculate_dissolve_instantly_fee(&self, amount_to_dissolve: Nat) -> Nat {
        amount_to_dissolve
            .scale_e8s_mul_f64(GLDT_STAKE_EARLY_UNSTAKE_FEE_PERCENTAGE)
            .scale_e8s_down()
    }

    // NOTE: all events are considered completed
    pub fn amount_available_for_withdraw(&self) -> Nat {
        self.dissolve_events
            .iter()
            .map(|event| event.amount.clone())
            .fold(Nat::from(0u32), |acc, amount| acc + amount)
    }

    // NOTE: all events are considered completed
    pub fn completed_dissolve_events(&self) -> Vec<DissolveStakeEvent> {
        self.dissolve_events.clone()
    }

    pub fn has_rewards(&self) -> bool {
        self.claimable_rewards
            .values()
            .any(|val| val > &Nat::from(0u64))
    }

    pub fn get_reward_by_token(&self, token: &TokenSymbol) -> Nat {
        self.claimable_rewards
            .get(token)
            .cloned()
            .unwrap_or(Nat::from(0u64))
    }

    fn update_age_bonus_timestamp(&mut self, amount_added: Nat) {
        let old_timestamp = self.age_bonus_timestamp;
        let denominator = self.staked.clone() + amount_added.clone();
        let numerator = self.staked.scaled_e8s_mul(old_timestamp)
            + amount_added.scaled_e8s_mul(timestamp_millis());
        self.age_bonus_timestamp = u64::try_from(
            &numerator
                .scaled_e8s_div(&denominator)
                .scale_e8s_down()
                .0
                .clone(),
        )
        .unwrap();
    }
}

pub enum StakeChange {
    Increase,
    Decrease(DecreaseType),
}

pub enum DecreaseType {
    Fractional,
    Full,
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn calculate_age_bonus_multiplier() {
        let now: TimestampMillis = timestamp_millis();
        let one_year_in_ms = (365.25 * DAY_IN_MS as f64) as u64; // Account for leap years

        // Create a position that was created 1 year ago
        let position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - one_year_in_ms,
            age_bonus_timestamp: (now - one_year_in_ms).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        let multiplier = position.calculate_age_bonus_multiplier(now);
        assert_eq!(multiplier, 2.0);
        let weighted_stake = position.calculate_weighted_stake(multiplier);
        assert_eq!(weighted_stake, Nat::from(2000u64));

        // Create a position that was created 6 months ago
        let position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - ((365.25 * DAY_IN_MS as f64) as u64 / 2),
            age_bonus_timestamp: (now - ((365.25 * DAY_IN_MS as f64) as u64 / 2)).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        let multiplier = position.calculate_age_bonus_multiplier(now);
        assert_eq!(multiplier, 1.5);
        let weighted_stake = position.calculate_weighted_stake(multiplier);
        assert_eq!(weighted_stake, Nat::from(1500u64));

        // Create a position that was created 0.5 days
        // it should get rounded down to 1.0
        let position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - (DAY_IN_MS / 2),
            age_bonus_timestamp: (now - (DAY_IN_MS / 2)).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        let multiplier = position.calculate_age_bonus_multiplier(now);
        assert_eq!(multiplier, 1.0);
        let weighted_stake = position.calculate_weighted_stake(multiplier);
        assert_eq!(weighted_stake, Nat::from(1000u64));

        // Create a position that was created 14 days ago
        let position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - (DAY_IN_MS * 14),
            age_bonus_timestamp: (now - (DAY_IN_MS * 14)).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        let multiplier = position.calculate_age_bonus_multiplier(now);
        let expected_multiplier = 1.0 + (14.0 * BONUS_INCREMENT);
        assert_eq!(multiplier, expected_multiplier);
        let weighted_stake = position.calculate_weighted_stake(multiplier);
        let expected_weighted_stake = Nat::from(((1.0 + (14.0 * BONUS_INCREMENT)) * 1000.0) as u64);
        assert_eq!(weighted_stake, expected_weighted_stake);

        // Create a position that was created 10 years ago
        // it should get rounded down to 1.0
        let nine_years = ((365.25 * 9.0) * DAY_IN_MS as f64) as u64;

        let position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - nine_years,
            age_bonus_timestamp: (now - nine_years).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        let multiplier = position.calculate_age_bonus_multiplier(now);
        let expected_multiplier = 10.0;
        assert_eq!(multiplier, expected_multiplier);
        let weighted_stake = position.calculate_weighted_stake(multiplier);
        assert_eq!(weighted_stake, Nat::from(10_000u64));
    }

    #[test]
    fn calculate_all_age_bonus_intervals_for_one_year() {
        const DAY_IN_MS: u64 = 86_400_000; // Milliseconds in a day
        const DAYS_IN_A_YEAR: f64 = 365.25; // Account for leap years
        const BONUS_INCREMENT: f64 = 1.0 / DAYS_IN_A_YEAR; // Daily increment
        let now: TimestampMillis = timestamp_millis();

        // Iterate over each day of the year (1 to 365)
        for day in 1..=365 {
            let position = StakePosition {
                owned_by: Principal::anonymous(),
                staked: Nat::from(1_000u64),
                created_at: now - (DAY_IN_MS * day), // Position created 'day' days ago
                age_bonus_timestamp: (now - (DAY_IN_MS * day)).into(), // Position created 'day' days ago
                claimable_rewards: HashMap::new(),
                dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
                claim_reward_status: ClaimRewardStatus::None,
                withdraw_state: WithdrawState::None,
                dissolve_events: Vec::default(),
            };

            let multiplier = position.calculate_age_bonus_multiplier(now);

            // Calculate the expected multiplier for the current day
            let expected_multiplier = 1.0 + BONUS_INCREMENT * (day as f64);
            if expected_multiplier != multiplier {
                println!("val: {multiplier} expected: {expected_multiplier}");
            }

            // Ensure the calculated multiplier is within an acceptable range
            assert!(
                (multiplier - expected_multiplier).abs() < 0.0001,
                "Day {day} failed: expected {expected_multiplier}, got {multiplier}"
            );
        }
    }

    #[test]
    fn calculate_age_bonus_multiplier_for_dissolving_position() {
        const DAY_IN_MS: u64 = 86_400_000; // Milliseconds in a day
        const DAYS_IN_A_YEAR: f64 = 365.25; // Account for leap years
        const BONUS_INCREMENT: f64 = 1.0 / DAYS_IN_A_YEAR; // Daily increment
        let now: TimestampMillis = timestamp_millis();

        // Iterate over each day of the year (1 to 365)
        for day in 1..=365 {
            let mut position = StakePosition {
                owned_by: Principal::anonymous(),
                staked: Nat::from(1_000u64),
                created_at: now - (DAY_IN_MS * day), // Position created 'day' days ago
                age_bonus_timestamp: (now - (DAY_IN_MS * day)).into(), // Position created 'day' days ago
                claimable_rewards: HashMap::new(),
                dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
                claim_reward_status: ClaimRewardStatus::None,
                withdraw_state: WithdrawState::None,
                dissolve_events: Vec::default(),
            };
            let _ = position.insert_event(DissolveStakeEvent {
                percentage: Percentage::new(99).unwrap(),
                amount: Nat::from(100_u64),
                dissolved_date: timestamp_millis() + DAY_IN_MS * 7,
                completed: false,
            });

            let multiplier = position.calculate_age_bonus_multiplier(now);

            // Calculate the expected multiplier for the current day
            let expected_multiplier = 1.0 + BONUS_INCREMENT * (day as f64);
            if expected_multiplier != multiplier {
                println!("val: {multiplier} expected: {expected_multiplier}");
            }

            // Ensure the calculated multiplier is within an acceptable range
            assert!(
                (multiplier - expected_multiplier).abs() < 0.0001,
                "Day {day} failed: expected {expected_multiplier}, got {multiplier}"
            );
        }
    }

    #[test]
    fn can_claim_reward() {
        // Create a position that was created 1 year ago
        let mut position =
            StakePosition::try_new(Principal::anonymous(), Nat::from(1_000_000_000_u64)).unwrap();

        position
            .claimable_rewards
            .insert(TokenSymbol::GOLDAO, Nat::from(1000u64));
        position
            .claimable_rewards
            .insert(TokenSymbol::ICP, Nat::from(1000u64));
        position
            .claimable_rewards
            .insert(TokenSymbol::OGY, Nat::from(1000u64));

        assert_eq!(
            position.claimable_rewards.get(&TokenSymbol::GOLDAO),
            Some(&Nat::from(1000u64))
        );
        assert_eq!(
            position.claimable_rewards.get(&TokenSymbol::ICP),
            Some(&Nat::from(1000u64))
        );
        assert_eq!(
            position.claimable_rewards.get(&TokenSymbol::OGY),
            Some(&Nat::from(1000u64))
        );

        let res = position.can_claim_reward(&TokenSymbol::GOLDAO, &Nat::from(1000u64));
        assert_matches!(res, Ok(_));

        let res = position.can_claim_reward(&TokenSymbol::GOLDAO, &Nat::from(1001u64));
        matches!(res, Err(RemoveRewardErrors::InsufficientBalance(_)));
    }

    #[test]
    fn adding_rewards_should_fail_if_position_is_dissolving() {
        // Create a position that was created 1 year ago
        let position =
            StakePosition::try_new(Principal::anonymous(), Nat::from(1_000_000_000_u64)).unwrap();

        assert_eq!(
            position.claimable_rewards.get(&TokenSymbol::GOLDAO),
            Some(&Nat::from(0u64))
        );

        let res = position.can_allocate_reward();
        matches!(res, Err(GeneralError::CannotAddReward(_)));

        assert_eq!(
            position.claimable_rewards.get(&TokenSymbol::GOLDAO),
            Some(&Nat::from(0u64))
        );
    }

    #[test]
    fn try_new_with_too_low_stake() {
        // Create a position that was created 1 year ago
        let position_res = StakePosition::try_new(Principal::anonymous(), Nat::from(1_000_u64));
        assert!(position_res.is_err())
    }

    #[test]
    fn test_calculate_new_reward() {
        let now: TimestampMillis = timestamp_millis();
        let total_weighted_stake_pool = Nat::from(10_000u64);
        let reward = Nat::from(10_000u64);
        let one_year = (365.25 * DAY_IN_MS as f64) as u64;

        // Create a position that was created 1 year ago
        let position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - one_year,
            age_bonus_timestamp: (now - one_year).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };
        let expected_reward = Nat::from(2000u64); // bonus modidier with a one year stake time should be 2.0

        let multiplier = position.calculate_age_bonus_multiplier(now);
        assert_eq!(multiplier, 2.0);
        let weighted_stake = position.calculate_weighted_stake(multiplier);
        assert_eq!(weighted_stake, Nat::from(2000u64));
        let position_reward =
            position.calculate_new_reward(&total_weighted_stake_pool, now, &reward);
        assert_eq!(position_reward, expected_reward)
    }

    #[test]
    fn test_calculate_instant_dissolve_fee() {
        let now: TimestampMillis = timestamp_millis();
        // Create a position that was created 1 year ago
        let position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(5_000u64),
            created_at: now - (DAY_IN_MS * 365),
            age_bonus_timestamp: (now - (DAY_IN_MS * 365)).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        let amount_to_dissolve = Nat::from(1_000u64);
        let expected_fee = Nat::from(0u64); // 0% of the initial stake
        assert_eq!(
            position.calculate_dissolve_instantly_fee(amount_to_dissolve),
            expected_fee
        )
    }

    #[test]
    fn test_can_withdraw() {
        let now: TimestampMillis = timestamp_millis();

        // Create a stake position from 1 year ago
        let mut position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - (DAY_IN_MS * 365),
            age_bonus_timestamp: (now - (DAY_IN_MS * 365)).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        // Case 1: No dissolve dissolve_events => cannot withdraw
        assert!(matches!(
            position.can_withdraw(),
            Err(WithdrawErrors::NoValidDissolveEvents(_))
        ));

        // Add some rewards
        position
            .claimable_rewards
            .insert(TokenSymbol::ICP, Nat::from(1000u64));

        // Add a dissolve event but simulate it's not yet matured
        let dissolved_date = now + DAY_IN_MS; // 1 day in the future
        let _ = position.dissolve_events.push(DissolveStakeEvent {
            percentage: Percentage::new(100).unwrap(),
            dissolved_date,
            completed: true,
            amount: Nat::from(1000_u64),
        });

        // Remove rewards
        position.claimable_rewards.remove(&TokenSymbol::ICP);

        // Case 2: Dissolve event not matured yet => still cannot withdraw
        assert!(matches!(position.can_withdraw(), Ok(())));

        // Simulate time after dissolve has matured by replacing with a past event
        let past_dissolve_date = now - DAY_IN_MS;
        let _ = position.dissolve_events.pop();
        let _ = position.dissolve_events.push(DissolveStakeEvent {
            percentage: Percentage::new(100).unwrap(),
            dissolved_date: past_dissolve_date,
            completed: true,
            amount: Nat::from(1000_u64),
        });

        // Case 3: Valid dissolve and no rewards => can withdraw
        assert_eq!(position.can_withdraw(), Ok(()));
    }

    #[test]
    pub fn test_dissolve_instantly() {
        let now: TimestampMillis = timestamp_millis();

        // Step 1: Create a stake position
        let mut position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - (DAY_IN_MS * 365),
            age_bonus_timestamp: (now - (DAY_IN_MS * 365)).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        // Step 2: Add rewards — early withdraw should fail
        position
            .claimable_rewards
            .insert(TokenSymbol::ICP, Nat::from(1000u64));

        assert!(matches!(
            position.can_dissolve_instantly(Percentage::new(100).unwrap()),
            Err(WithdrawErrors::CantWithdrawWithRewardsBalance(_))
        ));

        // Step 3: Remove rewards — still cannot early withdraw, no event
        position.claimable_rewards.remove(&TokenSymbol::ICP);

        // Step 4: Add an incomplete dissolve event — still cannot early withdraw
        let past_dissolve_date = now - DAY_IN_MS;
        let _ = position.dissolve_events.push(DissolveStakeEvent {
            percentage: Percentage::new(50).unwrap(),
            dissolved_date: past_dissolve_date, // in future
            completed: false,                   // not completed yet
            amount: Nat::from(500u64),
        });

        // Step 5: Replace with completed dissolve event
        let _ = position.dissolve_events.pop();
        let _event_id = position.dissolve_events.push(DissolveStakeEvent {
            percentage: Percentage::new(50).unwrap(),
            dissolved_date: now + DAY_IN_MS,
            completed: true,
            amount: Nat::from(500u64),
        });

        // Now early withdraw should succeed
        assert_eq!(
            position.can_dissolve_instantly(Percentage::new(100).unwrap()),
            Ok(())
        );
    }

    #[test]
    fn test_cleanup_completed_dissolve_events() {
        let mut position = StakePosition::try_new(
            Principal::anonymous(),
            Nat::from(MINIMUM_STAKE_AMOUNT_WITH_FEE + 1_000_000),
        )
        .unwrap();

        let now = timestamp_millis();
        println!("now: {:?}", now);

        let mut dissolve_events = vec![
            DissolveStakeEvent {
                percentage: Percentage::new(50).unwrap(),
                amount: Nat::from(MINIMUM_STAKE_AMOUNT_WITH_FEE + 1_000_000),
                dissolved_date: now - 1000, // already passed
                completed: false,
            },
            DissolveStakeEvent {
                percentage: Percentage::new(50).unwrap(),
                amount: Nat::from(MINIMUM_STAKE_AMOUNT_WITH_FEE + 1_000_000),
                dissolved_date: now + 1000, // in future
                completed: false,
            },
        ];

        for payload in dissolve_events.drain(..) {
            position.insert_event(payload);
        }

        // Only one of the dissolve_events should be marked completed
        position.cleanup_completed_dissolve_events();
        assert_eq!(position.dissolve_events.len(), 0);
    }

    #[test]
    fn test_has_rewards() {
        let now: TimestampMillis = timestamp_millis();

        // Create a position that was created 1 year ago
        let mut position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - (DAY_IN_MS * 365),
            age_bonus_timestamp: (now - (DAY_IN_MS * 365)).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        assert_eq!(position.has_rewards(), false);
        let _ = position
            .claimable_rewards
            .insert(TokenSymbol::ICP, Nat::from(1000u64));
        assert_eq!(position.has_rewards(), true);

        let _ = position.claimable_rewards.remove(&TokenSymbol::ICP);
        assert_eq!(position.has_rewards(), false);
    }

    #[test]
    pub fn test_update_age_bonus_timestamp() {
        let now: TimestampMillis = timestamp_millis();

        let staked = Nat::from(100_000_000_000_000u64); // 1'000'000 GLDT

        // Create a position that was created 1 year ago
        let mut position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: staked.clone(),
            created_at: now - (DAY_IN_MS * 365),
            age_bonus_timestamp: (now - (DAY_IN_MS * 365)).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        assert_eq!(position.age_bonus_timestamp, now - (DAY_IN_MS * 365));

        let factor = 3u64;

        position.update_age_bonus_timestamp(factor.clone() * staked);

        println!(
            "age_bonus_timestamp after: {:?}",
            position.age_bonus_timestamp
        );
        assert!(position.age_bonus_timestamp < now);

        let expected_timestamp = (now - (DAY_IN_MS * 365) + factor.clone() * now) / (factor + 1u64);
        assert_eq!(position.age_bonus_timestamp, expected_timestamp);
    }

    #[test]
    pub fn test_increase_stake() {
        let now: TimestampMillis = timestamp_millis();

        let staked = Nat::from(100_000_000_000u64); // 1'000 GLDT

        // Create a position that was created 1 year ago
        let mut position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: staked.clone(),
            created_at: now - (DAY_IN_MS * 365),
            age_bonus_timestamp: (now - (DAY_IN_MS * 365)).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };
        println!(
            "age_bonus_timestamp before: {:?}",
            position.age_bonus_timestamp
        );

        let factor = 3u64;
        position
            .change_stake(factor.clone() * staked.clone(), StakeChange::Increase)
            .unwrap();

        assert_eq!(position.staked, staked.clone() * (1u64 + factor));

        println!(
            "age_bonus_timestamp after: {:?}",
            position.age_bonus_timestamp
        );

        let expected_timestamp = (now - (DAY_IN_MS * 365) + factor.clone() * now) / (factor + 1u64);
        assert_eq!(position.age_bonus_timestamp, expected_timestamp);
    }

    #[test]
    pub fn test_stake_position_size() {
        let now: TimestampMillis = timestamp_millis();

        let staked = Nat::from(100_000_000_000u64);

        let mut position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: staked.clone(),
            created_at: now - (DAY_IN_MS * 365),
            age_bonus_timestamp: (now - (DAY_IN_MS * 365)).into(),
            claimable_rewards: HashMap::new(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            claim_reward_status: ClaimRewardStatus::None,
            withdraw_state: WithdrawState::None,
            dissolve_events: Vec::default(),
        };

        position.insert_event(DissolveStakeEvent {
            percentage: Percentage::new(50).unwrap(),
            amount: Nat::from(50_000_000_000u64),
            dissolved_date: now + DAY_IN_MS,
            completed: false,
        });

        // Check the size of the position
        let size = std::mem::size_of_val(&position);
        println!("Size of StakePosition: {} bytes", size);

        // Ensure the size is within a reasonable range
        assert!(size > 0 && size < 1024); // Example range, adjust as needed
    }
}
