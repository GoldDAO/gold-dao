use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::stake_position::UnstakeErrors;

// -------------------
// Claim Reward Event
// -------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, CandidType)]
pub enum ClaimRewardStatus {
    None,
    InProgress,
    Failed(String),
}

// -------------------
// Unstake Event
// -------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, CandidType)]
pub enum UnstakeStatus {
    None,
    InProgress,
    Failed(String),
    Unstaked,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, CandidType)]
pub enum NormalUnstakeStatus {
    None,
    InProgress,
    Failed(String),
    Unstaked,
}

// -------------------
// Unstake Early Event
// -------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, CandidType)]
pub enum UnstakeEarlyStatus {
    None,
    InProgress,
    Failed(String),
    UnstakedEarly,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, CandidType)]
pub enum UnstakeState {
    None,
    NormalUnstake(NormalUnstakeStatus),
    EarlyUnstake(UnstakeEarlyStatus),
}

impl UnstakeState {
    pub fn is_normal_unstake_event(&self) -> bool {
        match self {
            Self::None => false,
            Self::NormalUnstake(_) => true,
            Self::EarlyUnstake(_) => false,
        }
    }

    pub fn is_early_unstake_event(&self) -> bool {
        match self {
            Self::None => false,
            Self::NormalUnstake(_) => false,
            Self::EarlyUnstake(_) => true,
        }
    }
    pub fn is_unstaked(&self) -> bool {
        match self {
            Self::None => false,
            Self::NormalUnstake(_) => true,
            Self::EarlyUnstake(_) => true,
        }
    }
    pub fn is_valid_state_to_unstake(&self) -> Result<(), UnstakeErrors> {
        match self {
            UnstakeState::None => Ok(()),
            UnstakeState::NormalUnstake(normal_unstake_status) => match normal_unstake_status {
                NormalUnstakeStatus::None => Ok(()),
                NormalUnstakeStatus::InProgress => Err(UnstakeErrors::AlreadyProcessing(
                    "unstake procedure is already in progress".to_string(),
                )),
                NormalUnstakeStatus::Failed(_) => Ok(()),
                NormalUnstakeStatus::Unstaked => Err(UnstakeErrors::AlreadyUnstaked(
                    "position already is already unstaked".to_string(),
                )),
            },
            UnstakeState::EarlyUnstake(unstake_early_status) => match unstake_early_status {
                UnstakeEarlyStatus::None => Ok(()),
                UnstakeEarlyStatus::InProgress => Err(UnstakeErrors::AlreadyProcessing(
                    "early unstake procedure is already in progress".to_string(),
                )),
                UnstakeEarlyStatus::Failed(_) => Ok(()),
                UnstakeEarlyStatus::UnstakedEarly => Err(UnstakeErrors::AlreadyUnstaked(
                    "position already is already unstaked early".to_string(),
                )),
            },
        }
    }
}

impl Default for UnstakeState {
    fn default() -> Self {
        Self::None
    }
}
