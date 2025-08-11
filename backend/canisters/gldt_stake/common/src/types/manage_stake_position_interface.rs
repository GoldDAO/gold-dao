use candid::{CandidType, Nat};
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use serde::{Deserialize, Serialize};
use types::TokenSymbol;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ManageStakePositionArgs {
    AddStake { amount: Nat },
    ClaimRewards { tokens: Vec<TokenSymbol> },
    StartDissolving { fraction: u8 },
    DissolveInstantly { fraction: u8 },
    Withdraw {},
}

impl ManageStakePositionArgs {
    pub fn btype_name(&self) -> &'static str {
        match self {
            ManageStakePositionArgs::AddStake { .. } => "add_stake",
            ManageStakePositionArgs::ClaimRewards { .. } => "claim_rewards",
            ManageStakePositionArgs::StartDissolving { .. } => "start_dissolving",
            ManageStakePositionArgs::DissolveInstantly { .. } => "dissolve_instantly",
            ManageStakePositionArgs::Withdraw {} => "withdraw",
        }
    }
}

use std::collections::BTreeMap;
impl From<ManageStakePositionArgs> for ICRC3Value {
    fn from(args: ManageStakePositionArgs) -> Self {
        let mut map = BTreeMap::new();

        match args {
            ManageStakePositionArgs::AddStake { amount } => {
                map.insert("type".to_string(), ICRC3Value::Text("AddStake".to_string()));
                map.insert("amount".to_string(), ICRC3Value::Nat(amount));
            }
            ManageStakePositionArgs::ClaimRewards { tokens } => {
                map.insert(
                    "type".to_string(),
                    ICRC3Value::Text("ClaimRewards".to_string()),
                );
                map.insert(
                    "tokens".to_string(),
                    ICRC3Value::Array(
                        tokens
                            .into_iter()
                            .map(|t| ICRC3Value::Text(format!("{:?}", t)))
                            .collect(),
                    ),
                );
            }
            ManageStakePositionArgs::StartDissolving { fraction } => {
                map.insert(
                    "type".to_string(),
                    ICRC3Value::Text("StartDissolving".to_string()),
                );
                map.insert("fraction".to_string(), ICRC3Value::Nat(Nat::from(fraction)));
            }
            ManageStakePositionArgs::DissolveInstantly { fraction } => {
                map.insert(
                    "type".to_string(),
                    ICRC3Value::Text("DissolveInstantly".to_string()),
                );
                map.insert("fraction".to_string(), ICRC3Value::Nat(Nat::from(fraction)));
            }
            ManageStakePositionArgs::Withdraw {} => {
                map.insert("type".to_string(), ICRC3Value::Text("Withdraw".to_string()));
            }
        }

        ICRC3Value::Map(map)
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ManageStakePositionError {
    GeneralError(GeneralError),
    AddStakeError(AddStakePositionErrors),
    ClaimRewardError(Vec<ClaimRewardErrors>),
    StartDissolvingError(StartDissolvingErrors),
    DissolveInstantlyError(DissolveInstantlyRequestErrors),
    WithdrawError(WithdrawRequestErrors),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum GeneralError {
    AlreadyProcessing(String),
    CallError(String),
    InvalidPrincipal(String),
    StakePositionNotFound(String),
    NotAuthorized(String),
    ModifyStakeError(String),
    CannotAddReward(String),
    TransferError(String),
    InvalidPercentage(String),
    TransactionPreparationError(String),
    TransactionAddError(String),
    BalanceIsLowerThanFee(String),
    BalanceIsLowerThanThreshold(String),
    BalanceIsZero(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum WithdrawErrors {
    CantWithdrawWithRewardsBalance(String),
    InvalidDissolveState(String),
    AlreadyProcessing(String),
    NoValidDissolveEvents(String),
    InvalidWithdrawAmount(String),
    InvalidDissolveInstantlyAmount(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum RemoveRewardErrors {
    InsufficientBalance(String),
    RewardTokenTypeDoesNotExist(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum AddStakePositionErrors {
    CapacityExceeded(String),
    MaxAllowedStakePositions(String),
    StakePositionAlreadyExists(String),
    InvalidPrincipal(String),
    InvalidStakeAmount(String),
    TransferError(String),
    CallError(String),
    AlreadyProcessing(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum IncreaseStakePositionErrors {
    PositionNotFound(String),
    PositionDissolving(String),
    PositionDissolved(String),
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
    DissolvementsLimitReached(String),
    NotFound(String),
    AlreadyProcessing(String),
    InvalidDissolveAmount(String),
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
    NoTokensProvided(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum WithdrawRequestErrors {
    InvalidPrincipal(String),
    NotAuthorized(String),
    WithdrawErrors(WithdrawErrors),
    NotFound(String),
    TransferError(String),
    CallError(String),
    AlreadyWithdrawn(String),
    InvalidState(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum DissolveInstantlyRequestErrors {
    InvalidPrincipal(String),
    NotAuthorized(String),
    WithdrawErrors(WithdrawErrors),
    NotFound(String),
    TransferError(String),
    CallError(String),
    AlreadyProcessing(String),
    AlreadyWithdrawnEarly(String),
}

// Top-level conversions into ManageStakePositionError
impl From<GeneralError> for ManageStakePositionError {
    fn from(e: GeneralError) -> Self {
        ManageStakePositionError::GeneralError(e)
    }
}

impl From<AddStakePositionErrors> for ManageStakePositionError {
    fn from(e: AddStakePositionErrors) -> Self {
        ManageStakePositionError::AddStakeError(e)
    }
}

impl From<ClaimRewardErrors> for ManageStakePositionError {
    fn from(e: ClaimRewardErrors) -> Self {
        ManageStakePositionError::ClaimRewardError(vec![e])
    }
}

impl From<StartDissolvingErrors> for ManageStakePositionError {
    fn from(e: StartDissolvingErrors) -> Self {
        ManageStakePositionError::StartDissolvingError(e)
    }
}

impl From<WithdrawRequestErrors> for ManageStakePositionError {
    fn from(e: WithdrawRequestErrors) -> Self {
        ManageStakePositionError::WithdrawError(e)
    }
}

impl From<DissolveInstantlyRequestErrors> for ManageStakePositionError {
    fn from(e: DissolveInstantlyRequestErrors) -> Self {
        ManageStakePositionError::DissolveInstantlyError(e)
    }
}

impl From<WithdrawErrors> for WithdrawRequestErrors {
    fn from(e: WithdrawErrors) -> Self {
        WithdrawRequestErrors::WithdrawErrors(e)
    }
}

impl From<WithdrawErrors> for DissolveInstantlyRequestErrors {
    fn from(e: WithdrawErrors) -> Self {
        DissolveInstantlyRequestErrors::WithdrawErrors(e)
    }
}
