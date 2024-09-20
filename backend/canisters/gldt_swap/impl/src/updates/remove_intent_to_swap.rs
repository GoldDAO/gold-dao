use gldt_swap_api_canister::remove_intent_to_swap::RemoveIntentToSwapError;
use gldt_swap_common::swap::{ SwapInfo, SwapStatusForward };
pub use gldt_swap_api_canister::remove_intent_to_swap::{
    Args as RemoveIntentToSwapArgs,
    Response as RemoveIntentToSwapResponse,
};
use ic_cdk::update;
use utils::env::Environment;

use crate::{ state::{ mutate_state, read_state }, utils::is_nft_in_sale_state };

#[update]
async fn remove_intent_to_swap(args: RemoveIntentToSwapArgs) -> RemoveIntentToSwapResponse {
    let caller = read_state(|s| s.env.caller());

    if let Some(swap) = read_state(|s| s.data.swaps.get_active_swap(&args).cloned()) {
        if let SwapInfo::Forward(details) = &swap {
            // check there is no active sale
            if is_nft_in_sale_state(&details.nft_id_string, &details.nft_canister).await {
                return Err(RemoveIntentToSwapError::InProgress);
            }
            // check the caller is the user of the swap
            if details.gldt_receiver.owner != caller {
                return Err(RemoveIntentToSwapError::InvalidUser);
            }
            // they may only remove intent during the Init stage
            // a cron job takes care of the other stages
            if details.status != SwapStatusForward::Init {
                return Err(RemoveIntentToSwapError::InProgress);
            }
            mutate_state(|s| s.data.swaps.remove_swap_from_active_swaps(&args));
            Ok(())
        } else {
            Err(
                RemoveIntentToSwapError::InvalidSwapType(
                    format!("You may only remove forward swaps")
                )
            )
        }
    } else {
        Err(RemoveIntentToSwapError::SwapNotFound)
    }
}
