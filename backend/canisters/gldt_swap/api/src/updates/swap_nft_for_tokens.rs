use super::swap_tokens_for_nft::RetryInMilliseconds;
use candid::CandidType;
use gldt_swap_common::general_error::GeneralError;
use gldt_swap_common::nft::Nft;
use gldt_swap_common::swap::SwapIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub type Args = HashSet<Nft>;
pub type Response = Result<Vec<SwapIndex>, SwapNftForTokensErrors>;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum SwapNftForTokensErrors {
    Limit(String),
    CantBeAnonymous(String),
    Retry(RetryInMilliseconds),
    GeneralError(GeneralError),
}

impl From<GeneralError> for SwapNftForTokensErrors {
    fn from(err: GeneralError) -> Self {
        SwapNftForTokensErrors::GeneralError(err)
    }
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone, PartialEq, Eq)]
pub enum NftInvalidError {
    InvalidNFTCollectionPrincipal,
    InvalidTokenAmount,
    AlreadyLocked,
    InvalidNftOwner(String),
    GeneralError,
}
