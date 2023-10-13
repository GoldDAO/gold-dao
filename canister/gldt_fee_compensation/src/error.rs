use serde::Serialize;
use candid::{ CandidType, Deserialize };

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct CustomError {
    error_type: ErrorType,
    error_message: Option<String>,
}

impl CustomError {
    pub fn new(error_type: ErrorType) -> Self {
        Self { error_type, error_message: None }
    }

    pub fn new_with_message(error_type: ErrorType, error_message: String) -> Self {
        Self { error_type, error_message: Some(error_message) }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub enum ErrorType {
    /// An error during transfering the funds occured.
    TransferError,
    /// The specified royalty fee is not the expected one.
    InvalidRoyaltyFee,
    /// Invalid caller
    Unauthorized,
    /// Extensible error types
    Other,
}
