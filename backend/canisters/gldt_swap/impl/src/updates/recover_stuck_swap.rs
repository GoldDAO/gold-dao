use gldt_swap_api_canister::recover_stuck_swap::RecoverSwapError;
use gldt_swap_common::swap::{SwapInfo, SwapStatus, SwapStatusForward, SwapStatusReverse};

use crate::guards::caller_is_authorized;
pub use gldt_swap_api_canister::recover_stuck_swap::{
    Args as RecoverStuckSwapArgs, Response as RecoverStuckSwapResponse,
};
pub use gldt_swap_common::nft::NftID;
use ic_cdk::update;

use crate::swap::forward_swap::forward_swap_perform_deposit_recovery;
use crate::swap::swap_info::SwapInfoTrait;
use crate::{
    state::read_state,
    swap::reverse_swap::{burn_gldt, refund, transfer_fees, transfer_nft, transfer_to_escrow},
};

#[update(hidden = true, guard = "caller_is_authorized")]
async fn recover_stuck_swap(swap_id: RecoverStuckSwapArgs) -> RecoverStuckSwapResponse {
    recover_stuck_swap_impl(swap_id).await
}
pub async fn recover_stuck_swap_impl(swap_id: RecoverStuckSwapArgs) -> RecoverStuckSwapResponse {
    if let Some(swap) = read_state(|s| s.data.swaps.get_active_swap(&swap_id).cloned()) {
        // check if swap is stuck
        if !swap.is_swap_over_time_threshold() {
            return Err(RecoverSwapError::SwapIsNotStuck);
        }
        // process it again
        match &swap {
            SwapInfo::Reverse(details) => {
                match &details.status {
                    SwapStatusReverse::EscrowRequestInProgress => {
                        swap.update_status(SwapStatus::Reverse(SwapStatusReverse::EscrowRequest));
                    }
                    SwapStatusReverse::NftTransferRequestInProgress => {
                        swap.update_status(SwapStatus::Reverse(
                            SwapStatusReverse::NftTransferRequest,
                        ));
                    }
                    SwapStatusReverse::RefundRequestInProgress
                    | SwapStatusReverse::RefundFailed(_) => {
                        swap.update_status(SwapStatus::Reverse(SwapStatusReverse::RefundRequest));
                    }

                    SwapStatusReverse::BurnRequestInProgress | SwapStatusReverse::BurnFailed(_) => {
                        swap.update_status(SwapStatus::Reverse(SwapStatusReverse::BurnRequest));
                    }
                    SwapStatusReverse::FeeTransferRequestInProgress => {
                        swap.update_status(SwapStatus::Reverse(
                            SwapStatusReverse::FeeTransferRequest,
                        ));
                    }
                    _ => {}
                }

                transfer_to_escrow(&swap_id).await;
                transfer_nft(&swap_id).await;
                burn_gldt(&swap_id).await;
                transfer_fees(&swap_id).await;
                refund(&swap_id).await;
                Ok(swap_id)
            }
            SwapInfo::Forward(details) => {
                if matches!(
                    &details.status,
                    SwapStatusForward::DepositRecoveryRequest(_)
                        | SwapStatusForward::DepositRecoveryFailed(_, _)
                        | SwapStatusForward::DepositRecoveryInProgress(_)
                ) {
                    match &details.status {
                        SwapStatusForward::DepositRecoveryRequest(_) => {
                            swap.update_status(SwapStatus::Forward(
                                SwapStatusForward::DepositRecoveryRequest(Box::new(
                                    details.status.clone(),
                                )),
                            ));
                        }
                        SwapStatusForward::DepositRecoveryInProgress(_) => {
                            swap.update_status(SwapStatus::Forward(
                                SwapStatusForward::DepositRecoveryRequest(Box::new(
                                    details.status.clone(),
                                )),
                            ));
                        }
                        SwapStatusForward::DepositRecoveryFailed(swap_status_forward, _) => {
                            swap.update_status(SwapStatus::Forward(
                                SwapStatusForward::DepositRecoveryRequest(
                                    swap_status_forward.clone(),
                                ),
                            ));
                        }
                        _ => {}
                    }

                    match forward_swap_perform_deposit_recovery(&swap.get_swap_id()).await {
                        Ok(_) => {
                            return Ok(swap_id);
                        }
                        Err(_) => {
                            return Ok(swap_id);
                        }
                    }
                } else {
                    Err(
                        RecoverSwapError::InvalidForwardSwapType(
                            format!(
                                "You may only recover a forward swap that is DepositRecoveryRequest, DepositRecoveryFailed or DepositRecoveryInProgress. All others should be correctly handled and expired accordingly"
                            )
                        )
                    )
                }
            }
        }
    } else {
        Err(RecoverSwapError::NoSwapExists) // no active swap with this id
    }
}
