use candid::{ Nat, Principal };
use canister_time::timestamp_nanos;
use futures::future::join_all;
use gldt_swap_api_canister::get_historic_swaps_by_user::GetHistoricSwapsByUserError;
use gldt_swap_common::{ archive::ArchiveCanister, swap::{ SwapId, SwapInfo } };
use gldt_swap_archive_c2c_client::{ get_archive_swap, get_swap_indexes_for_user };

use ic_cdk::call;
use icrc_ledger_types::icrc1::{ account::{ Account, Subaccount }, transfer::{ Memo, TransferArg } };
use origyn_nft_reference::origyn_nft_reference_canister::{
    AuctionStateSharedStatus,
    NftInfoResult,
    SaleStatusSharedSaleType,
};
use origyn_nft_reference_c2c_client::nft_origyn;
use utils::env::Environment;

use crate::state::read_state;

pub async fn transfer_token(
    from_sub_account: Subaccount,
    to_account: Account,
    ledger_id: Principal,
    amount: Nat,
    memo: Option<Memo>
) -> Result<(), String> {
    match
        icrc_ledger_canister_c2c_client::icrc1_transfer(
            ledger_id,
            &(TransferArg {
                from_subaccount: Some(from_sub_account),
                to: to_account,
                fee: None,
                created_at_time: Some(timestamp_nanos()),
                amount: amount,
                memo: memo,
            })
        ).await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(error)) => Err(format!("Transfer error: {error:?}")),
        Err(error) => Err(format!("Network error: {error:?}")),
    }
}

pub fn trace(msg: &str) {
    unsafe {
        ic0::debug_print(msg.as_ptr() as i32, msg.len() as i32);
    }
}

pub async fn is_nft_in_sale_state(nft_id_string: &String, nft_canister_id: &Principal) -> bool {
    match nft_origyn(nft_canister_id.clone(), nft_id_string).await {
        Ok(res) => {
            match res {
                NftInfoResult::Ok(res_ok) => {
                    match res_ok.current_sale {
                        Some(current_sale) => {
                            match current_sale.sale_type {
                                SaleStatusSharedSaleType::Auction(auction) => {
                                    match auction.status {
                                        AuctionStateSharedStatus::Closed => false,
                                        AuctionStateSharedStatus::Open => true,
                                        AuctionStateSharedStatus::NotStarted => false,
                                    }
                                }
                            }
                        }
                        None => false,
                    }
                }
                NftInfoResult::Err(_) => false,
            }
        }
        Err(_) => { false }
    }
}

pub async fn get_all_user_swap_ids(
    user: &Principal
) -> Result<Vec<(SwapId, ArchiveCanister)>, GetHistoricSwapsByUserError> {
    let mut all_user_swap_ids: Vec<(SwapId, ArchiveCanister)> = Vec::new();

    let archives = read_state(|s| {
        let mut archives = s.data.swaps.get_archive_canisters().clone();
        // archives = archives
        //     .into_iter()
        //     .filter(|archive| archive.active)
        //     .collect();
        archives.sort_by_key(|canister| canister.start_index.clone());
        archives.reverse();
        archives
    });

    // Create a list of futures
    let (futures, archive_pair): (Vec<_>, Vec<_>) = archives
        .iter()
        .map(|archive| (get_swap_indexes_for_user(archive.canister_id, user), archive))
        .unzip();

    // Wait for all futures to complete
    let results = join_all(futures).await;

    // Process the results
    for (res, archive) in results.into_iter().zip(archive_pair.into_iter()) {
        match res {
            Ok(swap_ids) => {
                if let Some(ids) = swap_ids {
                    let with_archive_can: Vec<(SwapId, ArchiveCanister)> = ids
                        .into_iter()
                        .map(|id| (id, archive.clone()))
                        .collect();
                    all_user_swap_ids.extend(with_archive_can);
                }
            }
            Err(e) => {
                return Err(GetHistoricSwapsByUserError::QueryCanisterError(format!("{e:?}")));
            }
        }
    }

    Ok(all_user_swap_ids)
}

pub async fn get_historic_swap(swap_id: &SwapId) -> Option<SwapInfo> {
    let archive_canister = match
        read_state(|s| s.data.swaps.find_canister_for_swap_index(swap_id.1.clone()))
    {
        Some(canister_id) => canister_id,
        None => {
            return None;
        }
    };
    match get_archive_swap(archive_canister, swap_id).await {
        Ok(swap) => {
            match swap {
                Some((_, swap_info)) => Some(swap_info),
                None => None,
            }
        }
        Err(_) => None,
    }
}

pub async fn commit_changes() {
    let this_canister_id = read_state(|s| s.env.canister_id());
    let _ = ic_cdk::call::<(), ()>(this_canister_id, "commit", ()).await;
}
