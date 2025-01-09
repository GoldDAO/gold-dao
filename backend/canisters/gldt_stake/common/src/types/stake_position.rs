use candid::{CandidType, Decode, Encode, Nat, Principal};
use canister_time::{timestamp_millis, DAY_IN_MS};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{borrow::Cow, collections::HashMap};
use types::TimestampMillis;

use super::reward_tokens::TokenSymbol;
use super::stake_position_event::{
    ClaimRewardStatus, UnstakeEarlyStatus, UnstakeState, UnstakeStatus,
};
use super::{ledgers::GLDT_TX_FEE, numeric::ScaledArithmetic, reward_tokens::RewardTokens};

pub type StakePositionId = u64;

pub const GLDT_STAKE_MAX_ACTIVE_STAKE_POSITIONS_PER_USER: usize = 10;
pub const GLDT_STAKE_DISSOLVE_DELAY_MS: u64 = DAY_IN_MS * 7;
pub const GLDT_STAKE_EARLY_UNSTAKE_FEE_PERCENTAGE: f64 = 0.05;
pub const MINIMUM_STAKE_AMOUNT: u64 = 1_000_000_000;
pub const MINIMUM_STAKE_AMOUNT_WITH_FEE: u64 = MINIMUM_STAKE_AMOUNT + GLDT_TX_FEE;
pub const DAYS_IN_A_YEAR: f64 = 365.25; // Account for leap years
pub const BONUS_INCREMENT: f64 = 1.0 / DAYS_IN_A_YEAR; // Daily increment

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct StakePosition {
    /// The user who owns the stake position
    pub owned_by: Principal,
    /// The amount initially staked by the user
    pub staked: Nat,
    /// The timestamp when the position was created
    pub created_at: TimestampMillis,
    /// Rewards that can be claimed    
    pub claimable_rewards: RewardTokens,
    /// The dissolve status of the stake position
    pub dissolve_state: DissolveState,
    /// the delay in millisecnds that must be waited until a position can be unstaked
    pub dissolve_delay: Duration,
    /// if set, the datetime in milliseconds when the stake position will be fully dissolved and ready to be unstaked
    pub dissolved_date: Option<TimestampMillis>,
    /// status for claim reward procedure
    pub claim_reward_status: ClaimRewardStatus,
    /// status of unstaked ( both normal and early unstaking )
    pub unstake_state: UnstakeState,
}

impl Storable for StakePosition {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl StakePosition {
    pub fn new(owner: Principal, initial_stake_amount: Nat) -> Self {
        let mut claimable_rewards = HashMap::new();
        claimable_rewards.insert("ICP".to_string(), Nat::from(0u64));
        claimable_rewards.insert("OGY".to_string(), Nat::from(0u64));
        claimable_rewards.insert("GLDGov".to_string(), Nat::from(0u64));
        Self {
            owned_by: owner,
            staked: initial_stake_amount,
            created_at: timestamp_millis(),
            claimable_rewards: claimable_rewards,
            dissolve_state: DissolveState::default(),
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        }
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
        let percentage_scaled = position_weighted_stake.scaled_e8s_div(&total_weighted_stake_pool);
        let reward = (reward_pool_amount.clone() * percentage_scaled).scale_e8s_down();
        reward
    }

    pub fn calculate_age_bonus_multiplier(&self, current_timestamp: TimestampMillis) -> f64 {
        // Handle when dissolve state is dissolving - it should not increase, otherwise calculate from the current date
        let age_in_millis = match self.dissolved_date {
            Some(_) => return 0.0,
            None => current_timestamp.saturating_sub(self.created_at),
        };

        // Convert age in milliseconds to age in days
        let age_in_days = age_in_millis as f64 / DAY_IN_MS as f64;

        // Calculate the raw bonus increment
        let raw_bonus = age_in_days * BONUS_INCREMENT;

        // If less than 1 full day has passed, the multiplier should stay at 1.0
        let final_bonus = if age_in_days >= 1.0 {
            // If we've passed at least one full day, apply the raw bonus increment
            1.0 + raw_bonus
        } else {
            // Otherwise, return the base multiplier (1.0)
            1.0
        };

        final_bonus
    }

    pub fn can_start_dissolving(&self) -> Result<(), StakePositionError> {
        if self.dissolve_state != DissolveState::NotDissolving {
            return Err(StakePositionError::StartDissolvingError(format!(
                "stake position state must be NotDissolving but was found to be {:?}",
                self.dissolve_state
            )));
        }
        Ok(())
    }

    pub fn prepare_start_dissolving(&mut self) -> Result<(), StakePositionError> {
        if self.dissolve_state != DissolveState::NotDissolving {
            return Err(StakePositionError::StartDissolvingError(format!(
                "stake position state must be NotDissolving but was found to be {:?}",
                self.dissolve_state
            )));
        }
        self.dissolve_state = DissolveState::Dissolving;
        self.dissolved_date = Some(timestamp_millis() + self.dissolve_delay.as_millis() as u64);
        Ok(())
    }

    pub fn can_add_reward(&mut self) -> Result<(), StakePositionError> {
        if self.dissolve_state != DissolveState::NotDissolving {
            return Err(StakePositionError::AddRewardError(format!(
                "Can't add to position because dissolve state is {:?}",
                self.dissolve_state
            )));
        }
        Ok(())
    }

    pub fn can_claim_reward(&self, token: &String, amount: &Nat) -> Result<(), RemoveRewardErrors> {
        if let Some(current_rewards) = self.claimable_rewards.get(token) {
            if amount > current_rewards || current_rewards == &Nat::from(0u64) {
                Err(RemoveRewardErrors::InsufficientBalance(format!(
                    "cant deduct a reward of {} {} because balance is {} {}",
                    amount, token, current_rewards, token
                )))
            } else {
                Ok(())
            }
        } else {
            Err(RemoveRewardErrors::RewardTokenTypeDoesNotExist(format!(
                "Token of type '{}' does not exist in the rewards map",
                token
            )))
        }
    }

    pub fn can_unstake_early(&self) -> Result<(), UnstakeErrors> {
        if self.dissolve_state != DissolveState::NotDissolving {
            return Err(UnstakeErrors::InvalidDissolveState(format!(
                "The stake position has dissolve state of {:?} but needs to be {:?}",
                self.dissolve_state,
                DissolveState::NotDissolving
            )));
        }
        self.unstake_state.is_valid_state_to_unstake()?;
        if self.has_rewards() {
            return Err(UnstakeErrors::CantUnstakeWithRewardsBalance(format!("This stake position has rewards available to claim. The stake position must claim all rewards before unstaking")));
        }

        Ok(())
    }

    pub fn calculate_unstake_early_fee(&self) -> Nat {
        self.staked
            .scale_e8s_mul_f64(GLDT_STAKE_EARLY_UNSTAKE_FEE_PERCENTAGE)
            .scale_e8s_down()
    }

    pub fn can_unstake(&self) -> Result<(), UnstakeErrors> {
        let now = timestamp_millis();

        if self.dissolve_state != DissolveState::Dissolving {
            return Err(UnstakeErrors::InvalidDissolveState(format!(
                "The stake position has dissolve state of {:?} but needs to be {:?}",
                self.dissolve_state,
                DissolveState::Dissolving
            )));
        }

        self.unstake_state.is_valid_state_to_unstake()?;

        match self.dissolved_date {
            Some(dissolve_date) => {
                if now >= dissolve_date {
                    if self.has_rewards() {
                        return Err(UnstakeErrors::CantUnstakeWithRewardsBalance(format!("This stake position has rewards available to claim. The stake position must claim all rewards before unstaking")));
                    }
                    Ok(())
                } else {
                    Err(UnstakeErrors::DissolveDateNotSatisfied(format!("The stake position has a dissolve date of {} and this is less than the current time {}", dissolve_date, now)))
                }
            }
            None => Err(UnstakeErrors::NoDissolveDateSet(format!(
                "The stake position has no dissolve date"
            ))),
        }
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

    pub fn eligible_for_reward_allocation(&self) -> bool {
        self.dissolve_state == DissolveState::NotDissolving
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DissolveState {
    NotDissolving,
    Dissolving,
    Dissolved,
}

impl Default for DissolveState {
    fn default() -> Self {
        Self::NotDissolving
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum StakePositionError {
    AddRewardError(String),
    StartDissolvingError(String),
    UnStakeError(UnstakeErrors),
    RemoveRewardError(RemoveRewardErrors),
    AddStakePositionError(AddStakePositionErrors),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum UnstakeErrors {
    CantUnstakeWithRewardsBalance(String),
    InvalidDissolveState(String),
    DissolveDateNotSatisfied(String),
    NoDissolveDateSet(String),
    AlreadyProcessing(String),
    AlreadyUnstaked(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum RemoveRewardErrors {
    InsufficientBalance(String),
    RewardTokenTypeDoesNotExist(String),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum AddStakePositionErrors {
    MaxActiveStakePositions(String),
    InvalidPrincipal(String),
    InvalidStakeAmount(String),
    TransferError(String),
    CallError(String),
    AlreadyProcessing(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum StartDissolvingErrors {
    InvalidPrincipal(String),
    NotAuthorized(String),
    StakePositionError(StakePositionError),
    NotFound(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum ClaimRewardErrors {
    AlreadyProcessing(String),
    InvalidPrincipal(String),
    NotAuthorized(String),
    TokenImbalance(String),
    NotFound(String),
    InvalidRewardToken(String),
    TransferError(String),
    CallError(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum UnstakeRequestErrors {
    InvalidPrincipal(String),
    NotAuthorized(String),
    UnstakeErrors(UnstakeErrors),
    NotFound(String),
    TransferError(String),
    CallError(String),
    AlreadyUnstaked(String),
    InvalidState(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum UnstakeEarlyRequestErrors {
    InvalidPrincipal(String),
    NotAuthorized(String),
    UnstakeErrors(UnstakeErrors),
    NotFound(String),
    TransferError(String),
    CallError(String),
    AlreadyProcessing(String),
    AlreadyUnstakedEarly(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub struct StakePositionResponse {
    /// Id of the neuron
    pub id: StakePositionId,
    /// The user who owns the stake position
    pub owned_by: Principal,
    /// The amount initially staked by the user
    pub staked: Nat,
    /// The timestamp when the position was created
    pub created_at: TimestampMillis,
    /// An age bonus starting at 1 and increasing linearly by 1 year year
    pub age_bonus_multiplier: f64,
    /// The weighted stake considering the bonus multiplier
    pub weighted_stake: Nat,
    /// Rewards that can be claimed    
    pub claimable_rewards: HashMap<String, Nat>,
    /// The dissolve status of the stake position
    pub dissolve_state: DissolveState,
    /// the delay in millisecnds that must be waited until a position can be unstaked
    pub dissolve_delay: Duration,
    /// if set, the datetime in milliseconds when the stake position will be fully dissolved and ready to be unstaked
    pub dissolved_date: Option<TimestampMillis>,
    /// the fee associated with performing an early unstake instead of dissolving
    pub early_unstake_fee: Nat,
}

impl From<(StakePosition, TimestampMillis, StakePositionId)> for StakePositionResponse {
    fn from((position, timestamp, id): (StakePosition, TimestampMillis, StakePositionId)) -> Self {
        let age_bonus_multiplier = position.calculate_age_bonus_multiplier(timestamp);
        let weighted_stake = position.calculate_weighted_stake(age_bonus_multiplier);
        let early_unstake_fee = position.calculate_unstake_early_fee();
        StakePositionResponse {
            id,
            owned_by: position.owned_by,
            staked: position.staked,
            created_at: position.created_at,
            claimable_rewards: position.claimable_rewards,
            dissolve_state: position.dissolve_state,
            dissolve_delay: position.dissolve_delay,
            dissolved_date: position.dissolved_date,
            age_bonus_multiplier,
            weighted_stake,
            early_unstake_fee: early_unstake_fee,
        }
    }
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
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
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
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
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
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
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
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
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
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
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
                claimable_rewards: HashMap::new(),
                dissolve_state: DissolveState::NotDissolving,
                dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
                dissolved_date: None,
                claim_reward_status: ClaimRewardStatus::None,

                unstake_state: UnstakeState::None,
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
        // once the position starts to dissolve, the bonus multiplier should not increase
        let now: TimestampMillis = timestamp_millis();
        let one_year = (365.25 * DAY_IN_MS as f64) as u64;

        // Create a position that was created 1 year ago
        let mut position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - one_year,
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        };

        let _ = position.prepare_start_dissolving();

        let multiplier = position.calculate_age_bonus_multiplier(now);
        assert_eq!(multiplier, 0.0);
        let weighted_stake = position.calculate_weighted_stake(multiplier);
        assert_eq!(weighted_stake, Nat::from(0u64));

        // in 7 days it should still be 2.0
        let now_later = now + (DAY_IN_MS * 3);

        let multiplier = position.calculate_age_bonus_multiplier(now_later);
        assert_eq!(multiplier, 0.0);
        let weighted_stake = position.calculate_weighted_stake(multiplier);
        assert_eq!(weighted_stake, Nat::from(0u64));

        // in 30 days it should still be 2.0
        let now_later = now + (DAY_IN_MS * 30);

        let multiplier = position.calculate_age_bonus_multiplier(now_later);
        assert_eq!(multiplier, 0.0);
        let weighted_stake = position.calculate_weighted_stake(multiplier);
        assert_eq!(weighted_stake, Nat::from(0u64));

        // in 180 days it should still be 2.0
        let now_later = now + (DAY_IN_MS * 180);

        let multiplier = position.calculate_age_bonus_multiplier(now_later);
        assert_eq!(multiplier, 0.0);
        let weighted_stake = position.calculate_weighted_stake(multiplier);
        assert_eq!(weighted_stake, Nat::from(0u64));
    }

    #[test]
    fn can_claim_reward() {
        // Create a position that was created 1 year ago
        let mut position = StakePosition::new(Principal::anonymous(), Nat::from(1000u64));

        position
            .claimable_rewards
            .insert(format!("GLDGov"), Nat::from(1000u64));
        position
            .claimable_rewards
            .insert(format!("ICP"), Nat::from(1000u64));
        position
            .claimable_rewards
            .insert(format!("OGY"), Nat::from(1000u64));

        assert_eq!(
            position.claimable_rewards.get("GLDGov"),
            Some(&Nat::from(1000u64))
        );
        assert_eq!(
            position.claimable_rewards.get("ICP"),
            Some(&Nat::from(1000u64))
        );
        assert_eq!(
            position.claimable_rewards.get("OGY"),
            Some(&Nat::from(1000u64))
        );

        let res = position.can_claim_reward(&"GLDGov".to_string(), &Nat::from(1000u64));
        assert_matches!(res, Ok(_));

        let res = position.can_claim_reward(&"GLDGov".to_string(), &Nat::from(1001u64));
        matches!(res, Err(RemoveRewardErrors::InsufficientBalance(_)));
    }

    #[test]
    fn adding_rewards_should_fail_if_position_is_dissolving() {
        // Create a position that was created 1 year ago
        let mut position = StakePosition::new(Principal::anonymous(), Nat::from(1000u64));
        let _ = position.prepare_start_dissolving();

        assert_eq!(
            position.claimable_rewards.get("GLDGov"),
            Some(&Nat::from(0u64))
        );

        let res = position.can_add_reward();
        matches!(res, Err(StakePositionError::StartDissolvingError(_)));

        assert_eq!(
            position.claimable_rewards.get("GLDGov"),
            Some(&Nat::from(0u64))
        );
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
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
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
    fn test_calculate_early_unstake_fee() {
        let now: TimestampMillis = timestamp_millis();
        // Create a position that was created 1 year ago
        let position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - (DAY_IN_MS * 365),
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        };
        let expected_fee = Nat::from(50u64); // 5% of the initial stake
        assert_eq!(position.calculate_unstake_early_fee(), expected_fee)
    }

    #[test]
    fn test_can_unstake() {
        let now: TimestampMillis = timestamp_millis();

        // Create a position that was created 1 year ago
        let mut position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - (DAY_IN_MS * 365),
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        };

        matches!(
            position.can_unstake(),
            Err(UnstakeErrors::InvalidDissolveState(_))
        );

        // add some rewards
        let _ = position
            .claimable_rewards
            .insert(format!("ICP"), Nat::from(1000u64));

        // start dissolving
        let _ = position.prepare_start_dissolving();
        assert_eq!(position.dissolve_state, DissolveState::Dissolving);
        matches!(
            position.can_unstake(),
            Err(UnstakeErrors::DissolveDateNotSatisfied(_))
        ); // we still have rewards so we cant unstake yet

        // simulate 7 days apssing
        position.dissolved_date = Some(now - GLDT_STAKE_DISSOLVE_DELAY_MS);
        matches!(
            position.can_unstake(),
            Err(UnstakeErrors::CantUnstakeWithRewardsBalance(_))
        ); // we still have rewards so we cant unstake yet

        let _ = position.claimable_rewards.remove("ICP");

        assert_eq!(position.can_unstake(), Ok(())); //
    }

    #[test]
    fn test_has_rewards() {
        let now: TimestampMillis = timestamp_millis();

        // Create a position that was created 1 year ago
        let mut position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - (DAY_IN_MS * 365),
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        };

        assert_eq!(position.has_rewards(), false);
        let _ = position
            .claimable_rewards
            .insert(format!("ICP"), Nat::from(1000u64));
        assert_eq!(position.has_rewards(), true);

        let _ = position.claimable_rewards.remove("ICP");
        assert_eq!(position.has_rewards(), false);
    }

    #[test]
    pub fn test_unstake_early() {
        let now: TimestampMillis = timestamp_millis();

        // Create a position that was created 1 year ago
        let mut position = StakePosition {
            owned_by: Principal::anonymous(),
            staked: Nat::from(1_000u64),
            created_at: now - (DAY_IN_MS * 365),
            claimable_rewards: HashMap::new(),
            dissolve_state: DissolveState::NotDissolving,
            dissolve_delay: Duration::from_millis(GLDT_STAKE_DISSOLVE_DELAY_MS),
            dissolved_date: None,
            claim_reward_status: ClaimRewardStatus::None,
            unstake_state: UnstakeState::None,
        };

        // add some rewards
        let _ = position
            .claimable_rewards
            .insert(format!("ICP"), Nat::from(1000u64));

        matches!(
            position.can_unstake_early(),
            Err(UnstakeErrors::CantUnstakeWithRewardsBalance(_))
        ); // we still have rewards so we cant unstake yet

        let _ = position.claimable_rewards.remove("ICP");
    }
}
