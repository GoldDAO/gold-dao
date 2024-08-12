use candid::CandidType;
use serde::{Deserialize, Serialize};

mod queries;
mod updates;

pub use queries::*;
pub use updates::*;

pub type ICPSwapResult<T> = ResultLowercase<T, ICPSwapError>;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ICPSwapError {
    CommonError,
    InternalError(String),
    UnsupportedToken(String),
    InsufficientFunds,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ResultLowercase<T, E> {
    #[serde(rename = "ok")]
    Ok(T),
    #[serde(rename = "err")]
    Err(E),
}
