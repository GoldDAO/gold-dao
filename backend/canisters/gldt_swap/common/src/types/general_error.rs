use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum GeneralError {
    // Inter-canister error
    CallError(String),
    TransferError(String),

    // Internal canister error
    EmptyArgs(String),
    AlreadyProcessing(String),
    InvalidPrincipal(String),
    NotAuthorized(String),
    InvalidPercentage(String),
    InvalidNftCanister(String),
    UserIsNotNftOwner(String),
    InvalidConfig(String),
    ConfigNotFound(String),
    CanisterIsNotNftOwner(String),

    // ICRC3 error
    TransactionPreparationError(String),
    TransactionAddError(String),
}

use std::fmt;

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (kind, msg) = match self {
            GeneralError::EmptyArgs(msg) => ("EmptyArgs", msg),
            GeneralError::CallError(msg) => ("CallError", msg),
            GeneralError::TransferError(msg) => ("TransferError", msg),
            GeneralError::AlreadyProcessing(msg) => ("AlreadyProcessing", msg),
            GeneralError::InvalidPrincipal(msg) => ("InvalidPrincipal", msg),
            GeneralError::NotAuthorized(msg) => ("NotAuthorized", msg),
            GeneralError::InvalidPercentage(msg) => ("InvalidPercentage", msg),
            GeneralError::TransactionPreparationError(msg) => ("TransactionPreparationError", msg),
            GeneralError::TransactionAddError(msg) => ("TransactionAddError", msg),
            GeneralError::InvalidNftCanister(msg) => ("InvalidNftCanister", msg),
            GeneralError::UserIsNotNftOwner(msg) => ("UserIsNotNftOwner", msg),
            GeneralError::InvalidConfig(msg) => ("InvalidConfig", msg),
            GeneralError::ConfigNotFound(msg) => ("ConfigNotFound", msg),
            GeneralError::CanisterIsNotNftOwner(msg) => ("CanisterIsNotNftOwner", msg),
        };
        write!(f, "{}: {}", kind, msg)
    }
}
