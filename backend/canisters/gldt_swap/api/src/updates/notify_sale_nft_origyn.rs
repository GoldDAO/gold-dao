use candid::{ CandidType, Principal };
use serde::Deserialize;

use origyn_nft_reference::origyn_nft_reference_canister::{
    Account as NftSellerAccount,
    SaleStatusShared,
    SubAccountInfo,
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Args {
    pub escrow_info: SubAccountInfo,
    pub sale: SaleStatusShared,
    pub seller: NftSellerAccount,
    pub collection: Principal,
}

pub type Response = ();
