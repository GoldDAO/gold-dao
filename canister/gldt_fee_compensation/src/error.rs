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

    pub fn new_with_message(error_type: ErrorType, error_message: &str) -> Self {
        Self { error_type, error_message: Some(error_message.to_string()) }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub enum ErrorType {
    /// The minting of GLDT failed.
    TransferError,
    /// The swapping of NFT for GLDT failed.
    WrongFeeAmount,
    /// Invalid caller
    Unauthorized,
    /// Extensible error types
    Other,
}
