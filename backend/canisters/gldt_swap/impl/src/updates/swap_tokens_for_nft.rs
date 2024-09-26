use gldt_swap_common::swap::{ LockError, ServiceStatus, SwapStatus, SwapStatusReverse };
use gldt_swap_api_canister::swap_tokens_for_nft::SwapTokensForNftRequestErrors;
pub use gldt_swap_api_canister::swap_tokens_for_nft::{
    Args as SwapTokensForNftArgs,
    Response as SwapTokensForNftResponse,
};
use ic_cdk::update;
use tracing::debug;
use utils::env::Environment;
use crate::service_status::check_service_status;
use crate::swap::swap_info::SwapInfoTrait;

use crate::{
    state::read_state,
    swap::{
        reverse_swap::{ burn_gldt, refund, transfer_fees, transfer_nft, transfer_to_escrow },
        swap_builder::SwapBuilder,
    },
};

#[update]
async fn swap_tokens_for_nft(args: SwapTokensForNftArgs) -> SwapTokensForNftResponse {
    // check we have capacity to add new swaps
    if let ServiceStatus::Down(reason) = check_service_status().await {
        debug!("SERVICE Status :: down :: {reason:?}");
        return Err(SwapTokensForNftRequestErrors::ServiceDown(reason));
    }

    // 1. check if an active swap with the nft id already exists
    let caller = read_state(|s| s.env.caller());
    let nft_id = args.nft_id.clone();

    // 4. build a new swap - error early
    let new_swap = match SwapBuilder::reverse().init(&args, &caller).await {
        Ok(swap) => swap,
        Err((_, errors)) => {
            errors.iter().for_each(|e| {
                debug!(
                    "REVERSE SWAP :: swap_tokens_for_nft args are incorrect :: NFT = {nft_id:?} :: error(s) = {e:?}"
                );
            });
            return Err(SwapTokensForNftRequestErrors::NftValidationErrors(errors));
        }
    };

    // 5. insert the valid swap or return lock error if already active
    let swap_id = if let Ok(swap_id) = new_swap.insert_swap().await {
        swap_id
    } else {
        return Err(
            SwapTokensForNftRequestErrors::NftLocked(LockError::NftAlreadyLocked(vec![nft_id]))
        );
    };

    // 6. perform the swap and return the swap_id instantly
    if let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(&swap_id).cloned()) {
        swap_info.update_status(SwapStatus::Reverse(SwapStatusReverse::EscrowRequest));
        ic_cdk::spawn(async move {
            let swap_id = &swap_info.get_swap_id();
            transfer_to_escrow(&swap_id).await;
            transfer_nft(swap_id).await;
            burn_gldt(swap_id).await;
            transfer_fees(swap_id).await;
            refund(swap_id).await;
        });
        Ok(swap_id)
    } else {
        Err(SwapTokensForNftRequestErrors::SwapCreationError)
    }
}
