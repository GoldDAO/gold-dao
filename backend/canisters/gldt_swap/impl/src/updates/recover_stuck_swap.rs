use gldt_swap_common::swap::SwapInfo;
use gldt_swap_api_canister::recover_stuck_swap::RecoverSwapError;

pub use gldt_swap_api_canister::recover_stuck_swap::{
    Args as RecoverStuckSwapArgs,
    Response as RecoverStuckSwapResponse,
};
use ic_cdk::update;
pub use gldt_swap_common::nft::NftID;
use crate::guards::caller_is_authorized;

use crate::state::mutate_state;
use crate::swap::reverse_swap::{ disable_recovery_mode, enable_recovery_mode };
use crate::swap::swap_info::SwapInfoTrait;
use crate::{
    state::read_state,
    swap::reverse_swap::{ burn_gldt, refund, transfer_fees, transfer_nft, transfer_to_escrow },
};

#[update(hidden = true, guard = "caller_is_authorized")]
async fn recover_stuck_swap(swap_id: RecoverStuckSwapArgs) -> RecoverStuckSwapResponse {
    recover_stuck_swap_impl(swap_id).await
}
pub async fn recover_stuck_swap_impl(swap_id: RecoverStuckSwapArgs) -> RecoverStuckSwapResponse {
    if let Some(swap) = read_state(|s| s.data.swaps.get_active_swap(&swap_id).cloned()) {
        // check if swap is stuck
        if !swap.is_stuck() {
            return Err(RecoverSwapError::SwapIsNotStuck);
        }
        // process it again
        match swap {
            SwapInfo::Reverse(details) => {
                if details.in_recovery_mode {
                    return Err(RecoverSwapError::InProgress);
                }

                enable_recovery_mode(&swap_id);
                transfer_to_escrow(&swap_id).await;
                transfer_nft(&swap_id).await;
                burn_gldt(&swap_id).await;
                transfer_fees(&swap_id).await;
                refund(&swap_id).await;
                disable_recovery_mode(&swap_id);
                Ok(swap_id)
            }
            _ => { Err(RecoverSwapError::CantRecoverForwardSwaps) }
        }
    } else {
        Err(RecoverSwapError::NoSwapExists) // no active swap with this id
    }
}
