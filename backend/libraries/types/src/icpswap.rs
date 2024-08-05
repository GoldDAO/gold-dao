use crate::CanisterId;
use crate::Cryptocurrency;
use candid::CandidType;

use serde::{Deserialize, Serialize};
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ResultLowercase<T, E> {
    #[serde(rename = "ok")]
    Ok(T),
    #[serde(rename = "err")]
    Err(E),
}

// #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ICPSwapTokenInfo {
    pub token: Cryptocurrency,
    pub ledger: CanisterId,
    pub decimals: u8,
    pub fee: u128,
}
