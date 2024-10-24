#![allow(dead_code)] // Ignore warnings for unused code (functions, structs, etc.)
#![allow(unused_imports)] // Ignore warnings for unused imports
#![allow(unused_variables)] // Ignore warnings for unused variables
#![allow(unused_mut)] // Ignore warnings for unused mutable variables
#![allow(unused_macros)]

use gldt_swap_common::swap::{ trace, SwapInfo, SwapStatus, SwapStatusForward, SwapStatusReverse };
use ic_cdk::update;
pub use gldt_swap_api_canister::insert_fake_bulk_swaps::{
    Args as InsertFakeBulkSwapArgs,
    Response as InsertFakeBulkSwapResponse,
};
use crate::guards::caller_is_authorized;
use crate::swap::swap_info::SwapInfoTrait;
use gldt_swap_common::swap::MAX_SWAP_INFO_BYTES_SIZE;

#[cfg(feature = "inttest")]
#[update(hidden = true, guard = "caller_is_authorized")]
async fn insert_fake_bulk_swaps(swaps: InsertFakeBulkSwapArgs) -> InsertFakeBulkSwapResponse {
    insert_fake_bulk_swaps_impl(swaps).await
}

async fn insert_fake_bulk_swaps_impl(swaps: InsertFakeBulkSwapArgs) -> InsertFakeBulkSwapResponse {
    // valid and create new swaps - error swaps are saved too
    for swap in swaps {
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
                    Err(e) => {
                        return Err(format!("Something went wrong 1"));
                    }
                };
            }
            SwapInfo::Reverse(swap_details) => {
                let mut new_reverse_swap_status = swap_details.status.clone();
                swap_details.status = SwapStatusReverse::Init;
                let nft_id = swap_details.nft_id.clone();
                match new_swap.insert_swap().await {
                    Ok(swap_id) => {
                        new_swap.update_status(
                            SwapStatus::Reverse(new_reverse_swap_status.clone())
                        );
                    }
                    Err(_) => {
                        return Err(format!("Something went wrong 2"));
                    }
                };
            }
        }
    }

    Ok(())
}
