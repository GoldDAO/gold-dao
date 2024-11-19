use crate::service_status::check_service_status;
use crate::swap::swap_info::SwapInfoTrait;
use candid::Principal;
use canister_time::SECOND_IN_MS;
pub use gldt_swap_api_canister::swap_tokens_for_nft::{
    Args as SwapTokensForNftArgs, Response as SwapTokensForNftResponse,
};
use gldt_swap_api_canister::swap_tokens_for_nft::{
    RetryInMilliseconds, SwapTokensForNftRequestErrors,
};
use gldt_swap_common::swap::{
    LockError, ServiceDownReason, ServiceStatus, SwapStatus, SwapStatusReverse,
};
use ic_cdk::update;
use tracing::debug;
use utils::env::Environment;

use crate::utils::check_fee_account_has_enough_ogy;
use crate::{
    state::read_state,
    swap::{
        reverse_swap::{burn_gldt, refund, transfer_fees, transfer_nft, transfer_to_escrow},
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
    let ogy_base_fee = read_state(|s| s.data.base_ogy_swap_fee.clone());

    if caller == Principal::anonymous() {
        return Err(SwapTokensForNftRequestErrors::CantBeAnonymous(format!(
            "You can't use an annoymous principal to swap"
        )));
    }

    if !check_fee_account_has_enough_ogy(args.nft_canister_id, ogy_base_fee).await {
        return Err(SwapTokensForNftRequestErrors::ServiceDown(
            ServiceDownReason::LowOrigynToken(
                "Not enough OGY in the fee account for this swap to proceed".to_string(),
            ),
        ));
    }

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

    if read_state(|s| s.data.is_gldt_supply_balancer_running) {
        return Err(SwapTokensForNftRequestErrors::Retry(RetryInMilliseconds(
            SECOND_IN_MS * 30,
            format!("the supply is currently being balanced. please try again in 15 seconds"),
        )));
    }

    // 5. insert the valid swap or return lock error if already active
    let swap_id = if let Ok(swap_id) = new_swap.insert_swap().await {
        swap_id
    } else {
        return Err(SwapTokensForNftRequestErrors::NftLocked(
            LockError::NftAlreadyLocked(vec![nft_id]),
        ));
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
