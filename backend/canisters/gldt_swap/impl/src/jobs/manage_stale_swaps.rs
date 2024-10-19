/*!
# Force fail stuck forward swaps 
# due to the nature of how a forward swap works. we can't retry or recover swaps. The NFT canister will refund and cancel the sale after 1 minute
## Interval : 5 minutes
## forces, Init, Mint and Bid Requests to fail
*/

use crate::{
    state::{ mutate_state, read_state },
    swap::forward_swap::forward_swap_perform_burn_fees,
    utils::{ is_nft_in_sale_state, trace },
};
use canister_time::{ run_interval, MINUTE_IN_MS };
use futures::{ future::{ join_all, BoxFuture }, FutureExt };
use gldt_swap_common::{
    gldt::GldtTokenSpec,
    swap::{ SwapErrorForward, SwapId, SwapInfo, SwapStatus, SwapStatusForward },
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

    let swaps = get_active_swaps();
    let expired_swaps = filter_expired_swaps(swaps);

    for batch in expired_swaps.chunks(10) {
        let (futures, swaps) = filter_active_sales(batch).await;
        let results = join_all(futures).await;

        let swaps_to_recognise_escrow = classify_swaps(results, swaps).await;
        trace(&format!("/////////////////// 1 : {swaps_to_recognise_escrow:?}"));

        // validate escrow deposit exists and withdraw it
        let valid_escrow_deposits = recognize_escrow_transfers(swaps_to_recognise_escrow).await;
        trace(&format!("/////////////////// 2 : {valid_escrow_deposits:?}"));
        withdraw_escrow_transfers(valid_escrow_deposits).await;
    }

    mutate_state(|s| {
        s.data.is_remove_stale_swaps_cron_running = false;
    });
}

// Function to get active swaps (Vec<(SwapId, SwapInfo)>)
fn get_active_swaps() -> Vec<(SwapId, SwapInfo)> {
    read_state(|s| s.data.swaps.get_active_swaps())
}

// Function to filter expired swaps
fn filter_expired_swaps(swaps: Vec<(SwapId, SwapInfo)>) -> Vec<(SwapId, SwapInfo)> {
    swaps
        .into_iter()
        .filter(|(_, swap_info)| swap_info.is_stuck())
        .filter(|(_, swap_info)| {
            match swap_info {
                SwapInfo::Forward(swap_detail_forward) => {
                    !matches!(
                        swap_detail_forward.status,
                        SwapStatusForward::MintInProgress |
                            SwapStatusForward::BidInProgress |
                            SwapStatusForward::BurnFeesInProgress
                    )
                }
                SwapInfo::Reverse(_) => false,
            }
        })
        .collect()
}

// Function to filter active sales
async fn filter_active_sales(
    batch: &[(SwapId, SwapInfo)]
) -> (Vec<BoxFuture<'_, bool>>, Vec<SwapInfo>) {
    let futures_and_swaps: Vec<(BoxFuture<'_, bool>, SwapInfo)> = batch
        .iter()
        .filter_map(|(_, swap_info)| {
            match swap_info {
                SwapInfo::Forward(details) => {
                    let future = is_nft_in_sale_state(
                        &details.nft_id_string,
                        &details.nft_canister
                    ).boxed(); // Box the future
                    Some((future, swap_info.clone()))
                }
                _ => None,
            }
        })
        .collect();

    futures_and_swaps.into_iter().unzip()
}

// Classify swaps into escrow and withdrawal
async fn classify_swaps(results: Vec<bool>, swaps: Vec<SwapInfo>) -> Vec<SwapInfo> {
    let mut swaps_to_recognise_escrow = vec![];

    for (has_active_sale, swap_info) in results.into_iter().zip(swaps.into_iter()) {
        if !has_active_sale {
            if let SwapInfo::Forward(details) = &swap_info {
                match details.status {
                    SwapStatusForward::BidFail(_) | SwapStatusForward::BidRequest => {
                        swaps_to_recognise_escrow.push(swap_info);
                    }
                    SwapStatusForward::BurnFeesFailed(_) => {
                        retry_failed_burn_fees(&swap_info).await; // Awaiting async function
                    }
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

    swaps_to_recognise_escrow
}

// Retry burn fees for failed forward swaps
async fn retry_failed_burn_fees(swap_info: &SwapInfo) {
    swap_info.update_status(SwapStatus::Forward(SwapStatusForward::BurnFeesRequest));
    forward_swap_perform_burn_fees(&swap_info.get_swap_id()).await;
}
async fn recognize_escrow_transfers(swaps: Vec<SwapInfo>) -> Vec<SwapInfo> {
    let mut recognized_swaps = Vec::new(); // Store the successful ones
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());

    for swap_info in swaps.into_iter() {
        if let SwapInfo::Forward(details) = swap_info.clone() {
            let args = ManageSaleRequest::RecognizeEscrow(EscrowRequest {
                token_id: details.nft_id_string.clone(),
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
                    sale_id: Some(details.sale_id.clone()),
                },
                lock_to_date: None,
            });

            let manage_sale_result = sale_nft_origyn(details.nft_canister, args).await;

            if let Ok(ManageSaleResult::Ok(_)) = manage_sale_result {
                recognized_swaps.push(swap_info); // Only push successful ones
            } else {
                debug!("Failed to recognize escrow for swap.");
            }
        }
    }
    recognized_swaps // Return the successfully recognized swaps
}
async fn withdraw_escrow_transfers(swaps: Vec<SwapInfo>) {
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());

    for swap_info in swaps.into_iter() {
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

            let manage_sale_result = sale_nft_origyn(details.nft_canister, args).await;

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
                                debug!("STALE SWAP JOB : impossible branch - ");
                            }
                        }
                    }
                }
                Ok(ManageSaleResult::Err(e)) => {
                    let swap_id = swap_info.get_swap_id();
                    debug!(
                        "STALE SWAP JOB : Swap ID :: {swap_id:?} - Failed to withdraw forward swap deposit: {e:?}"
                    );
                }
                Err(e) => {
                    let swap_id = swap_info.get_swap_id();
                    debug!(
                        "STALE SWAP JOB : Swap ID :: {swap_id:?} - Failed to withdraw forward swap deposit: {e:?}"
                    );
                }
            }
        }
    }
}

// Handle sale results
// fn handle_sale_result(
//     result: Result<ManageSaleResult, SwapErrorForward>,
//     swap_info: SwapInfo,
//     swaps_to_withdraw: &mut Vec<SwapInfo>
// ) {
//     match result {
//         Ok(ManageSaleResult::Ok(_)) => {
//             swaps_to_withdraw.push(swap_info);
//         }
//         Ok(ManageSaleResult::Err(e)) => {
//             debug!("STALE SWAP JOB : Failed to process with error : {e:?}");
//         }
//         Err(e) => {
//             debug!("STALE SWAP JOB : Failed to process with error : {e:?}");
//         }
//     }
// }
