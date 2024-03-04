use serde::Serialize;
use candid::{ CandidType, Deserialize };

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Custom {
    error_type: Type,
    error_message: Option<String>,
}

impl Custom {
    pub fn new(error_type: Type) -> Self {
        Self { error_type, error_message: None }
    }

    pub fn new_with_message(error_type: Type, error_message: String) -> Self {
        Self { error_type, error_message: Some(error_message) }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub enum Type {
    /// An error during transfering the funds occured.
    TransferError,
    /// Invalid caller
    Unauthorized,
    /// Compensation disabled
    CompensationDisabled,
    /// Extensible error types
    Other,
}
