use candid::{ CandidType, Principal };
use serde::{ Deserialize, Serialize };

use gldt_swap_common::{
    nft::NftID,
    swap::{ LockError, NftValidationError, ServiceDownReason, SwapId },
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Args {
    pub nft_id: NftID,
    pub nft_canister_id: Principal,
}

pub type Response = Result<SwapId, SwapTokensForNftRequestErrors>;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum SwapTokensForNftRequestErrors {
    GetNftMetaDetailError(GetNftMetaDetailErrorReason),
    NftLocked(LockError),
    NftValidationErrors(Vec<NftValidationError>),
    CantForgeSwapId,
    ServiceDown(ServiceDownReason),
    SwapCreationError,
    NotOwnedBySwapCanister,
    CantBeAnonymous(String),
}

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
