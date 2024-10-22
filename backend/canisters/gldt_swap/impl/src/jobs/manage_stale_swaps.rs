use crate::{
    state::{ mutate_state, read_state },
    swap::forward_swap::{ forward_swap_perform_burn_fees, forward_swap_perform_deposit_recovery },
    utils::{ is_nft_in_sale_state, trace },
};
use canister_time::{ run_interval, MINUTE_IN_MS };
use futures::{ future::{ join_all, BoxFuture }, FutureExt };
use gldt_swap_common::{
    swap::{ SwapErrorForward, SwapId, SwapInfo, SwapStatus, SwapStatusForward },
};
use std::time::Duration;
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

    for batch in expired_swaps.chunks(45) {
        let (futures, swaps) = filter_active_sales(batch).await;
        let results = join_all(futures).await;

        let (swaps_to_recognise_escrow, swaps_to_retry_burn, swaps_to_auto_expire) = classify_swaps(
            results,
            swaps
        );

        auto_expire_swaps(swaps_to_auto_expire);
        withdraw_expired_swap_deposits(swaps_to_recognise_escrow).await;
        retry_failed_burn_fees(swaps_to_retry_burn).await;
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
        .filter(|(_, swap_info)| swap_info.is_swap_over_time_threshold())
        .filter(|(_, swap_info)| {
            match swap_info {
                SwapInfo::Forward(swap_detail_forward) => {
                    !matches!(
                        swap_detail_forward.status,
                        SwapStatusForward::MintInProgress |
                            SwapStatusForward::BidInProgress |
                            SwapStatusForward::BurnFeesInProgress |
                            SwapStatusForward::NotificationInProgress |
                            SwapStatusForward::DepositRecoveryInProgress(_)
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
fn classify_swaps(
    results: Vec<bool>,
    swaps: Vec<SwapInfo>
) -> (Vec<SwapInfo>, Vec<SwapInfo>, Vec<SwapInfo>) {
    let mut swaps_to_retry_burn: Vec<SwapInfo> = vec![];
    let mut swaps_to_recover_deposit: Vec<SwapInfo> = vec![];
    let mut swaps_to_auto_expire: Vec<SwapInfo> = vec![];
    for (has_active_sale, swap_info) in results.into_iter().zip(swaps.into_iter()) {
        if !has_active_sale {
            if let SwapInfo::Forward(details) = &swap_info {
                match details.status {
                    SwapStatusForward::BidFail(_) | SwapStatusForward::BidRequest => {
                        swaps_to_recover_deposit.push(swap_info);
                    }
                    SwapStatusForward::BurnFeesFailed(_) | SwapStatusForward::BurnFeesRequest => {
                        swaps_to_retry_burn.push(swap_info);
                    }
                    | SwapStatusForward::DepositRecoveryFailed(_, _)
                    | SwapStatusForward::DepositRecoveryRequest(_) => {}
                    _ => {
                        swaps_to_auto_expire.push(swap_info);
                    }
                }
            }
        }
    }

    (swaps_to_recover_deposit, swaps_to_retry_burn, swaps_to_auto_expire)
}

// Retry burn fees for failed forward swaps
async fn retry_failed_burn_fees(swaps: Vec<SwapInfo>) {
    let futures = swaps.into_iter().map(|swap| {
        swap.update_status(SwapStatus::Forward(SwapStatusForward::BurnFeesRequest));
        let swap_id = swap.get_swap_id().clone(); // Clone the swap_id
        async move { forward_swap_perform_burn_fees(&swap_id).await }
    });

    join_all(futures).await;
}

fn auto_expire_swaps(swaps: Vec<SwapInfo>) {
    for swap in swaps {
        swap.update_status(
            SwapStatus::Forward(SwapStatusForward::Failed(SwapErrorForward::Expired))
        );
    }
}
async fn withdraw_expired_swap_deposits(swaps: Vec<SwapInfo>) {
    let futures: Vec<_> = swaps
        .iter()
        .filter_map(|swap_info| {
            match &swap_info {
                SwapInfo::Forward(swap_detail_forward) => {
                    swap_info.update_status(
                        SwapStatus::Forward(
                            SwapStatusForward::DepositRecoveryRequest(
                                Box::new(swap_detail_forward.status.clone())
                            )
                        )
                    );
                    let id = swap_info.get_swap_id().clone();
                    Some(async move { forward_swap_perform_deposit_recovery(&id).await })
                }
                _ => { None }
            }
        })
        .collect();

    join_all(futures).await;
}
