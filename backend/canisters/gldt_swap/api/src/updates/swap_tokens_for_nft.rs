use candid::CandidType;
use gldt_swap_common::general_error::GeneralError;
use gldt_swap_common::nft::Nft;
use gldt_swap_common::swap::SwapIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub type Args = HashSet<Nft>;
pub type Response = Result<Vec<SwapIndex>, SwapTokensForNftErrors>;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum SwapTokensForNftErrors {
    Limit(String),
    SwapCreationError,
    NotOwnedBySwapCanister,
    Retry(RetryInMilliseconds),
    GeneralError(GeneralError),
}

impl From<GeneralError> for SwapTokensForNftErrors {
    fn from(err: GeneralError) -> Self {
        SwapTokensForNftErrors::GeneralError(err)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RetryInMilliseconds(pub u64, pub String);

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum GetNftMetaDetailErrorReason {
    UnexpectedError(String),
    NoMetaDetails,
    CantFindNFT(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum NftInvalidReason {
    InvalidWeight,
    InvalidNftCanisterForWeight,
}
