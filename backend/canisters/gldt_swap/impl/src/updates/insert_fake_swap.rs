#![allow(dead_code)] // Ignore warnings for unused code (functions, structs, etc.)
#![allow(unused_imports)] // Ignore warnings for unused imports
#![allow(unused_variables)] // Ignore warnings for unused variables
#![allow(unused_mut)] // Ignore warnings for unused mutable variables
#![allow(unused_macros)]

use gldt_swap_common::swap::{ SwapInfo, SwapStatus, SwapStatusForward, SwapStatusReverse };
use ic_cdk::update;
pub use gldt_swap_api_canister::insert_fake_swap::{
    Args as InsertFakeSwapArgs,
    Response as InsertFakeSwapResponse,
};
use crate::guards::caller_is_authorized;
use crate::swap::swap_info::SwapInfoTrait;

#[cfg(feature = "inttest")]
#[update(hidden = true, guard = "caller_is_authorized")]
async fn insert_fake_swap(swap: InsertFakeSwapArgs) -> InsertFakeSwapResponse {
    insert_fake_swap_impl(swap).await
}

async fn insert_fake_swap_impl(swap: InsertFakeSwapArgs) -> InsertFakeSwapResponse {
    // valid and create new swaps - error swaps are saved too
    let mut new_swap = swap.clone();

    match &mut new_swap {
        SwapInfo::Forward(swap_details) => {
            let mut new_forward_swap_status = swap_details.status.clone();
            swap_details.status = SwapStatusForward::Init;
            let nft_id = swap_details.nft_id.clone();
            match new_swap.insert_swap().await {
                Ok(swap_id) => {
                    new_swap.update_status(SwapStatus::Forward(new_forward_swap_status));
                }
                Err(_) => {}
            };
        }
        SwapInfo::Reverse(swap_details) => {
            let mut new_reverse_swap_status = swap_details.status.clone();
            swap_details.status = SwapStatusReverse::Init;
            let nft_id = swap_details.nft_id.clone();
            match new_swap.insert_swap().await {
                Ok(swap_id) => {
                    new_swap.update_status(SwapStatus::Reverse(new_reverse_swap_status.clone()));
                }
                Err(_) => {}
            };
        }
    }

    Ok(())
}
