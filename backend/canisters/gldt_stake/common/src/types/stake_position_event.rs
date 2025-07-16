use crate::manage_stake_position_interface::WithdrawErrors;
use candid::CandidType;
use candid::Nat;
use serde::{Deserialize, Serialize};
use types::TimestampMillis;
use utils::numeric::Percentage;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct DissolveStakeEvent {
    pub percentage: Percentage,
    pub amount: Nat,
    pub dissolved_date: TimestampMillis,
    pub completed: bool,
}

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
// Withdraw Event
// -------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, CandidType)]
pub enum NormalWithdrawStatus {
    None,
    InProgress,
    Failed(String),
    Withdrawn,
}

// -------------------
// Withdraw Early Event
// -------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, CandidType)]
pub enum DissolveInstantlyStatus {
    None,
    InProgress,
    Failed(String),
    DissolvedInstantly,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, CandidType)]
pub enum WithdrawState {
    None,
    NormalWithdraw(NormalWithdrawStatus),
    EarlyWithdraw(DissolveInstantlyStatus),
}

impl WithdrawState {
    pub fn is_valid_state_to_withdraw(&self) -> Result<(), WithdrawErrors> {
        match self {
            WithdrawState::None => Ok(()),
            WithdrawState::NormalWithdraw(normal_withdraw_status) => match normal_withdraw_status {
                NormalWithdrawStatus::None => Ok(()),
                NormalWithdrawStatus::Withdrawn => Ok(()),
                NormalWithdrawStatus::InProgress => Err(WithdrawErrors::AlreadyProcessing(
                    "withdraw procedure is already in progress".to_string(),
                )),
                NormalWithdrawStatus::Failed(_) => Ok(()),
            },
            WithdrawState::EarlyWithdraw(dissolve_instantly_status) => {
                match dissolve_instantly_status {
                    DissolveInstantlyStatus::None => Ok(()),
                    DissolveInstantlyStatus::DissolvedInstantly => Ok(()),
                    DissolveInstantlyStatus::InProgress => Err(WithdrawErrors::AlreadyProcessing(
                        "instant dissolve procedure is already in progress".to_string(),
                    )),
                    DissolveInstantlyStatus::Failed(_) => Ok(()),
                }
            }
        }
    }
}

impl Default for WithdrawState {
    fn default() -> Self {
        Self::None
    }
}
