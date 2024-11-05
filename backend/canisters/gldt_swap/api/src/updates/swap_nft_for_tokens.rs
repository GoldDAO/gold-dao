use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use gldt_swap_common::{
    nft::NftID,
    swap::{ServiceDownReason, SwapId},
};

pub type Args = Vec<(NftID, Principal)>;

pub type Response = Result<Vec<SwapId>, SwapNftForTokensErrors>;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum SwapNftForTokensErrors {
    NftValidationErrors((Vec<NftID>, Vec<(NftID, Vec<NftInvalidError>)>)),
    ContainsDuplicates(String),
    ContainsInvalidNftCanister(String),
    ServiceDown(ServiceDownReason),
    Limit(String),
    CantBeAnonymous(String),
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone, PartialEq, Eq)]
pub enum NftInvalidError {
    InvalidNFTCollectionPrincipal,
    CantGetNatIdOfNft,
    InvalidTokenAmount,
    AlreadyLocked,
    CantGetOrigynID(String),
    InvalidNftOwner(String),
    NftIdStringTooLong(String),
}
