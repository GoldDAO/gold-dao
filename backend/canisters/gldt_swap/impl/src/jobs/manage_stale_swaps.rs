/*!
# Force fail stuck forward swaps 
# due to the nature of how a forward swap works. we can't retry or recover swaps. The NFT canister will refund and cancel the sale after 1 minute
## Interval : 5 minutes
## forces, Init, Mint and Bid Requests to fail
*/

use crate::{
    state::{ mutate_state, read_state },
    swap::forward_swap::forward_swap_perform_burn_fees,
    utils::is_nft_in_sale_state,
};
use canister_time::{ run_interval, MINUTE_IN_MS };
use futures::future::join_all;
use gldt_swap_common::{
    gldt::GldtTokenSpec,
    swap::{ SwapErrorForward, SwapInfo, SwapStatus, SwapStatusForward },
};
use origyn_nft_reference::origyn_nft_reference_canister::{
    Account as OrigynAccount,
    DepositDetail,
    EscrowRequest,
    ManageSaleRequest,
    ManageSaleResult,
    WithdrawDescription,
    WithdrawRequest,
};
use origyn_nft_reference_c2c_client::sale_nft_origyn;
use utils::env::Environment;
use std::time::Duration;
use tracing::debug;
use types::Milliseconds;
use crate::swap::swap_info::SwapInfoTrait;

const INTERVAL: Milliseconds = MINUTE_IN_MS * 1;

pub fn start_job() {
    run_interval(Duration::from_millis(INTERVAL), || {
        let is_running = read_state(|s| s.data.is_remove_stale_swaps_cron_running);
        if is_running {
            return;
        }
        ic_cdk::spawn(handle_remove_stale_swap());
    });
}

async fn handle_remove_stale_swap() {
    mutate_state(|s| {
        s.data.is_remove_stale_swaps_cron_running = true;
    });
    let swaps = read_state(|s| s.data.swaps.get_active_swaps());
    let this_canister_id = read_state(|s| s.env.canister_id());
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);

    // filter swaps by expiry time
    let expired_swaps: Vec<_> = swaps
        .into_iter()
        .filter(|(_, swap_info)| swap_info.is_stuck())
        .collect();
    for batch in expired_swaps.chunks(10) {
        // **************************************
        //      Check there is an active sale
        // **************************************

        let (futures, swaps): (Vec<_>, Vec<_>) = batch
            .iter()
            .filter_map(|(_, swap_info)| {
                match swap_info {
                    SwapInfo::Forward(details) =>
                        Some((
                            is_nft_in_sale_state(&details.nft_id_string, &details.nft_canister),
                            swap_info.clone(),
                        )),
                    _ => None, // Handle only Forward swaps here
                }
            })
            .unzip();

        let results = join_all(futures).await;
        let mut swaps_to_recognise_escrow = vec![];
        let mut swaps_to_withdraw = vec![];

        for (has_active_sale, swap_info) in results.into_iter().zip(swaps.into_iter()) {
            if !has_active_sale {
                if let SwapInfo::Forward(details) = swap_info.clone() {
                    match details.status {
                        // if there is no active sale and the swap has these statuses then recognise their escrow transfers and process refunds
                        SwapStatusForward::BidFail(_) | SwapStatusForward::BidRequest => {
                            swaps_to_recognise_escrow.push(swap_info);
                        }

                        SwapStatusForward::BurnFeesFailed(_) => {
                            // retry the burn of fees
                            swap_info.update_status(
                                SwapStatus::Forward(SwapStatusForward::BurnFeesRequest)
                            );
                            forward_swap_perform_burn_fees(&swap_info.get_swap_id()).await;
                        }
                        // all other forward swaps can be auto expired.
                        _ => {
                            swap_info.update_status(
                                SwapStatus::Forward(
                                    SwapStatusForward::Failed(SwapErrorForward::Expired)
                                )
                            );
                        }
                    }
                }
            }
        }
        debug!("attmepint to cancel the following swap ids {swaps_to_recognise_escrow:?}");

        // *****************************************************
        //      Recognize an escrow transfer for forward swaps
        // - you must recognize an escrow transfer before you can withdraw
        // *****************************************************
        for (future, swap_info) in swaps_to_recognise_escrow.into_iter().filter_map(|swap_info| {
            if let SwapInfo::Forward(details) = swap_info.clone() {
                let args = ManageSaleRequest::RecognizeEscrow(EscrowRequest {
                    token_id: details.nft_id_string,
                    deposit: DepositDetail {
                        token: GldtTokenSpec::new(gldt_ledger_id).get_token_spec(),
                        trx_id: None,
                        seller: OrigynAccount::Account {
                            owner: details.gldt_receiver.owner,
                            sub_account: None,
                        },
                        buyer: OrigynAccount::Account {
                            owner: this_canister_id,
                            sub_account: None,
                        },
                        amount: details.tokens_to_mint.get_with_fee().clone(),
                        sale_id: Some(details.sale_id),
                    },
                    lock_to_date: None,
                });
                Some((sale_nft_origyn(details.nft_canister, args), swap_info))
            } else {
                debug!("Unexpected swap type encountered during escrow recognition.");
                None
            }
        }) {
            let manage_sale_result = future.await;
            match manage_sale_result {
                Ok(ManageSaleResult::Ok(_)) => {
                    swaps_to_withdraw.push(swap_info.clone());
                }
                Ok(ManageSaleResult::Err(e)) => {
                    debug!("STALE SWAP JOB : Failed to recognize escrow with error : {e:?}");
                }
                Err(e) => {
                    debug!("STALE SWAP JOB : Failed to recognize escrow with error : {e:?}");
                }
            }
        }

        // *****************************************************
        //      withdraw escrow deposit for recognized escrow deposits
        // - withdraw the recognized escrow deposits
        // *****************************************************
        for (future, swap_info) in swaps_to_withdraw.into_iter().filter_map(|swap_info| {
            if let SwapInfo::Forward(details) = swap_info.clone() {
                let args = ManageSaleRequest::Withdraw(
                    WithdrawRequest::Escrow(WithdrawDescription {
                        token: GldtTokenSpec::new(gldt_ledger_id).get_token_spec_with_no_fee(),
                        token_id: details.nft_id_string.clone(),
                        seller: OrigynAccount::Account {
                            owner: details.gldt_receiver.owner,
                            sub_account: None,
                        },
                        withdraw_to: OrigynAccount::Principal_(this_canister_id),
                        buyer: OrigynAccount::Account {
                            owner: this_canister_id,
                            sub_account: None,
                        },
                        amount: details.tokens_to_mint.get_with_fee(),
                    })
                );
                Some((sale_nft_origyn(details.nft_canister, args), swap_info))
            } else {
                debug!("Unexpected swap type encountered during withdrawal.");
                None
            }
        }) {
            let manage_sale_result = future.await;
            match manage_sale_result {
                Ok(ManageSaleResult::Ok(_)) => {
                    if let SwapInfo::Forward(details) = swap_info.clone() {
                        match details.status {
                            SwapStatusForward::BidRequest => {
                                let _ = &swap_info.update_status(
                                    SwapStatus::Forward(
                                        SwapStatusForward::Failed(SwapErrorForward::Expired)
                                    )
                                );
                            }
                            SwapStatusForward::BidFail(e) => {
                                let _ = &swap_info.update_status(
                                    SwapStatus::Forward(
                                        SwapStatusForward::Failed(SwapErrorForward::BidFailed(e))
                                    )
                                );
                            }
                            _ => {
                                debug!("STALE SWAP JOB : impossible branch");
                            }
                        }
                    }
                }
                Ok(ManageSaleResult::Err(e)) => {
                    debug!(
                        "STALE SWAP JOB : Failed to refund forward swap escrow with error : {e:?}"
                    );
                }
                Err(e) => {
                    debug!(
                        "STALE SWAP JOB : Failed to refund forward swap escrow with error : {e:?}"
                    );
                }
            }
        }
    }

    mutate_state(|s| {
        s.data.is_remove_stale_swaps_cron_running = false;
    });
}
