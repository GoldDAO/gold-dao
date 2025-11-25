use crate::guards::{reject_anonymous_caller, GuardNft};
use crate::model::nft_batches::GroupedNftsByCanister;
use crate::state::{icrc3_commit_prepared_transaction, mutate_state, read_state};
use crate::utils::prepare_transactions;
use bity_ic_canister_time::SECOND_IN_MS;
use candid::Principal;
pub use gldt_swap_api_canister::swap_tokens_for_nft::{
    Args as SwapTokensForNftArgs, Response as SwapTokensForNftResponse, RetryInMilliseconds,
    SwapTokensForNftErrors,
};
use gldt_swap_common::general_error::GeneralError;
use gldt_swap_common::gldt::GLDT_SWAP_FEE_ACCOUNT;
use gldt_swap_common::swap::{SwapIndex, SwapInfo, SwapStatus, SwapType};
use ic_cdk::update;
use icrc_ledger_canister_c2c_client::{icrc1_transfer, icrc2_transfer_from};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use icrc_ledger_types::icrc2::transfer_from::TransferFromArgs;
use std::collections::{HashMap, HashSet};
use tracing::{error, info};
use utils::env::Environment;

#[update]
async fn swap_tokens_for_nft(args: SwapTokensForNftArgs) -> SwapTokensForNftResponse {
    swap_tokens_for_nft_impl(args).await
}

pub async fn swap_tokens_for_nft_impl(args: SwapTokensForNftArgs) -> SwapTokensForNftResponse {
    let _span = tracing::info_span!("SWAP_TOKENS_FOR_NFT").entered();

    let caller = ic_cdk::api::msg_caller();

    reject_anonymous_caller()
        .map_err(|e| SwapTokensForNftErrors::GeneralError(GeneralError::InvalidPrincipal(e)))?;

    if args.is_empty() {
        return Err(SwapTokensForNftErrors::GeneralError(
            GeneralError::EmptyArgs("There were no NFTs provided for swap".to_string()),
        ));
    }

    if args.len() > 100 {
        return Err(SwapTokensForNftErrors::Limit(
            "You may only swap 100 in any given request. Batch your calls in batches of 100"
                .to_string(),
        ));
    }

    if read_state(|s| s.data.is_gldt_supply_balancer_running) {
        return Err(SwapTokensForNftErrors::Retry(RetryInMilliseconds(
            SECOND_IN_MS * 30,
            "The supply is currently being balanced. Please try again in ~15 seconds.".to_string(),
        )));
    }

    read_state(|s| s.data.swap_configs.validate_nfts(&args))?;

    let batched_nfts = GroupedNftsByCanister::from_nfts(args.clone());

    let canister_principal = ic_cdk::api::canister_self();
    batched_nfts
        .validate_all_owned_by(canister_principal)
        .await
        .into_iter()
        .find_map(|res| res.err())
        .map_or(Ok(()), |err| {
            Err(SwapTokensForNftErrors::GeneralError(
                GeneralError::UserIsNotNftOwner(err.to_string()),
            ))
        })?;

    let _nft_guards: Vec<_> = args
        .iter()
        .map(|nft| {
            GuardNft::new(nft.clone()).map_err(|e| {
                SwapTokensForNftErrors::GeneralError(GeneralError::AlreadyProcessing(e))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let tokens_amount: HashMap<Principal, candid::Nat> = batched_nfts
        .calculate_token_equivalent()
        .map_err(SwapTokensForNftErrors::GeneralError)?;
    let swap_fees: HashMap<Principal, candid::Nat> = batched_nfts
        .calculate_fees_amount()
        .map_err(SwapTokensForNftErrors::GeneralError)?;

    // 1. Create swaps
    let swaps = mutate_state(|s| {
        s.data.swap_system.create_swaps_batch(
            SwapType::Reverse,
            caller,
            args.clone(),
            &s.data.swap_configs,
        )
    })
    .map_err(|e| {
        error!("Failed to create swaps: {:?}", e);
        SwapTokensForNftErrors::GeneralError(e)
    })?;

    let swap_ids: Vec<candid::Nat> = swaps.keys().cloned().collect();

    let prepared_transactions = prepare_transactions(&swaps)?;

    // 2.0 Transfer fees
    // NOTE: if something fails further - the fee is not returned
    let transfer_args: Vec<_> = swap_fees
        .iter()
        .map(|(ledger_id, amount)| {
            (
                *ledger_id,
                TransferFromArgs {
                    spender_subaccount: None,
                    from: Account {
                        owner: caller,
                        subaccount: None,
                    },
                    to: Account {
                        owner: read_state(|s| s.env.canister_id()),
                        subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
                    },
                    fee: None,
                    created_at_time: None,
                    memo: None,
                    amount: amount.clone(),
                },
            )
        })
        .collect();

    let transfer_fee_futures = transfer_args
        .iter()
        .map(|(ledger_id, arg)| icrc2_transfer_from(*ledger_id, arg))
        .collect::<Vec<_>>();
    let transfer_fee_results = futures::future::join_all(transfer_fee_futures).await;
    ic_cdk::println!("transfer_fee_results: {:?}", transfer_fee_results);

    let mut transfer_fee_success: Vec<SwapIndex> = Vec::new();
    let mut transfer_fee_failed: Vec<SwapIndex> = Vec::new();
    let mut transfer_fee_fail_reasons: HashMap<SwapIndex, String> = HashMap::new();
    for ((ledger_id, _arg), result) in transfer_args.iter().zip(transfer_fee_results.into_iter()) {
        let swaps_for_ledger: Vec<&SwapInfo> = swaps
            .values()
            .filter(|s| s.tokens_amount.ledger_id == *ledger_id)
            .collect();
        for swap in swaps_for_ledger {
            match &result {
                Ok(icrc_ledger_canister::icrc2_transfer_from::Response::Ok(_amt)) => {
                    transfer_fee_success.push(swap.index.clone());
                }
                Ok(icrc_ledger_canister::icrc2_transfer_from::Response::Err(e)) => {
                    transfer_fee_failed.push(swap.index.clone());
                    transfer_fee_fail_reasons.insert(
                        swap.index.clone(),
                        format!("Token transfer failed: {:?}", e),
                    );
                }
                Err(call_err) => {
                    transfer_fee_failed.push(swap.index.clone());
                    transfer_fee_fail_reasons.insert(
                        swap.index.clone(),
                        format!("IC call failed during fee transfer: {:?}", call_err),
                    );
                }
            }
        }
    }

    mutate_state(|s| {
        let sys = &mut s.data.swap_system;
        sys.update_swaps_statuses(&transfer_fee_success, SwapStatus::Burned);
        for idx in &transfer_fee_failed {
            let reason = transfer_fee_fail_reasons
                .get(idx)
                .cloned()
                .unwrap_or_else(|| "Unknown burn failure".to_string());
            sys.update_swaps_status(idx, SwapStatus::BurnFailed(reason));
        }
    });

    if transfer_fee_success.is_empty() {
        return Err(SwapTokensForNftErrors::GeneralError(
            GeneralError::CallError("No token burns succeeded; NFTs not transferred.".into()),
        ));
    }

    // 2.1. Transfer tokens
    let transfer_args: Vec<_> = tokens_amount
        .iter()
        .map(|(ledger_id, amount)| {
            (
                *ledger_id,
                TransferFromArgs {
                    spender_subaccount: None,
                    from: Account {
                        owner: caller,
                        subaccount: None,
                    },
                    to: Account {
                        owner: read_state(|s| s.env.canister_id()),
                        subaccount: None,
                    },
                    fee: None,
                    created_at_time: None,
                    memo: None,
                    amount: amount.clone(),
                },
            )
        })
        .collect();

    let burn_futures = transfer_args
        .iter()
        .map(|(ledger_id, arg)| icrc2_transfer_from(*ledger_id, arg))
        .collect::<Vec<_>>();
    let burn_results = futures::future::join_all(burn_futures).await;
    ic_cdk::println!("burn_results: {:?}", burn_results);

    let mut burned_success: Vec<SwapIndex> = Vec::new();
    let mut burned_failed: Vec<SwapIndex> = Vec::new();
    let mut burn_fail_reasons: HashMap<SwapIndex, String> = HashMap::new();
    for ((ledger_id, _arg), result) in transfer_args.iter().zip(burn_results.into_iter()) {
        let swaps_for_ledger: Vec<&SwapInfo> = swaps
            .values()
            .filter(|s| s.tokens_amount.ledger_id == *ledger_id)
            .collect();
        for swap in swaps_for_ledger {
            match &result {
                Ok(icrc_ledger_canister::icrc2_transfer_from::Response::Ok(_amt)) => {
                    burned_success.push(swap.index.clone());
                }
                Ok(icrc_ledger_canister::icrc2_transfer_from::Response::Err(e)) => {
                    burned_failed.push(swap.index.clone());
                    burn_fail_reasons
                        .insert(swap.index.clone(), format!("Token burn failed: {:?}", e));
                }
                Err(call_err) => {
                    burned_failed.push(swap.index.clone());
                    burn_fail_reasons.insert(
                        swap.index.clone(),
                        format!("IC call failed during burn: {:?}", call_err),
                    );
                }
            }
        }
    }

    mutate_state(|s| {
        let sys = &mut s.data.swap_system;
        (&burned_success, SwapStatus::Burned);
        for idx in &burned_failed {
            let reason = burn_fail_reasons
                .get(idx)
                .cloned()
                .unwrap_or_else(|| "Unknown burn failure".to_string());
            sys.update_swaps_status(idx, SwapStatus::BurnFailed(reason));
        }
    });

    if burned_success.is_empty() {
        return Err(SwapTokensForNftErrors::GeneralError(
            GeneralError::CallError("No token burns succeeded; NFTs not transferred.".into()),
        ));
    }

    // 3. Transfer NFTs
    let successful_swaps_set: HashSet<SwapIndex> = burned_success.iter().cloned().collect();
    let nfts_for_transfer: Vec<_> = swaps
        .values()
        .filter(|s| successful_swaps_set.contains(&s.index))
        .map(|s| s.nft.clone())
        .collect();

    let grouped_to_transfer = GroupedNftsByCanister::from_nfts(nfts_for_transfer.clone());
    let nft_transfer_results = grouped_to_transfer.batch_transfer_all(caller).await;

    let mut nft_transfer_success: Vec<SwapIndex> = Vec::new();
    let mut nft_transfer_failed: HashMap<SwapIndex, String> = HashMap::new();

    for swap in swaps.values() {
        if let Some(results) = nft_transfer_results.get(&swap.nft.canister_id) {
            let nft_success: Result<bool, _> =
                results.iter().try_fold(false, |found, res| match res {
                    Ok(nft_id) if *nft_id == swap.nft.id => Ok(true),
                    Ok(_) => Ok(found),
                    Err(e) => Err(e),
                });

            match nft_success {
                Ok(true) => {
                    // successful_nfts.push(swap.nft.clone());
                    nft_transfer_success.push(swap.index.clone());
                }
                Ok(false) => {
                    nft_transfer_failed.insert(
                        swap.index.clone(),
                        "NFT not found in transfer results".to_string(),
                    );
                }
                Err(err) => {
                    nft_transfer_failed
                        .insert(swap.index.clone(), format!("Transfer error: {:?}", err));
                }
            }
        } else {
            nft_transfer_failed.insert(
                swap.index.clone(),
                "No transfer attempt for this canister".to_string(),
            );
        }
    }

    mutate_state(|s| {
        let sys = &mut s.data.swap_system;
        sys.update_swaps_statuses(&nft_transfer_success, SwapStatus::NftTransferred);
        for (idx, reason) in &nft_transfer_failed {
            sys.update_swaps_status(idx, SwapStatus::NftTransferFailed(reason.clone()));
        }
    });

    mutate_state(|s| {
        let sys = &mut s.data.swap_system;
        sys.update_swaps_statuses(&nft_transfer_success, SwapStatus::Minted);
        sys.update_swaps_statuses(&nft_transfer_success, SwapStatus::Complete);
    });

    let complete_swap_indices: Vec<_> = nft_transfer_success
        .iter()
        .filter_map(|idx| {
            let swap = read_state(|s| s.data.swap_system.swaps.get(idx).cloned())?;
            if swap.status == SwapStatus::Complete {
                Some((idx.clone(), swap))
            } else {
                None
            }
        })
        .collect();

    for (swap_index, _) in complete_swap_indices {
        let (transaction, prepared_transaction) = prepared_transactions.get(&swap_index).unwrap();
        if let Err(e) =
            icrc3_commit_prepared_transaction(transaction.clone(), prepared_transaction.timestamp)
        {
            ic_cdk::println!("Commit failed for swap_index {:?}: {:?}", swap_index, e);
            error!(
                "icrc3_commit_prepared_transaction failed for swap_index {:?}: {:?}",
                swap_index, e
            );
        } else {
            ic_cdk::println!(
                "Successfully committed transaction for swap_index {:?}",
                swap_index
            );
            info!(
                "Successfully committed transaction for swap_index {:?}",
                swap_index
            );
        }
    }

    // 4. Reimburse failed swaps
    let swaps_to_reimburse =
        read_state(|s| s.data.swap_system.filter_swaps_to_reimburse(&swap_ids));
    let nfts_for_reimburse: Vec<_> = swaps_to_reimburse.values().map(|s| s.nft.clone()).collect();

    let filtered_nfts = GroupedNftsByCanister::from_nfts(nfts_for_reimburse);

    let tokens_amount: HashMap<Principal, candid::Nat> = filtered_nfts
        .calculate_token_equivalent()
        .map_err(SwapTokensForNftErrors::GeneralError)?;

    let transfer_args: Vec<_> = tokens_amount
        .iter()
        .map(|(canister_id, amount)| {
            (
                *canister_id,
                TransferArg {
                    from_subaccount: None,
                    to: Account {
                        owner: caller,
                        subaccount: None,
                    },
                    fee: None,
                    created_at_time: None,
                    memo: None,
                    amount: amount.clone(),
                },
            )
        })
        .collect();

    let transfers_futures = transfer_args
        .iter()
        .map(|(token_canister_id, transfer_arg)| icrc1_transfer(*token_canister_id, transfer_arg))
        .collect::<Vec<_>>();
    let transfer_results = futures::future::join_all(transfers_futures).await;

    let mut successful_swap_ids = Vec::new();
    let mut failed_swap_ids: Vec<SwapIndex> = Vec::new();
    let mut failed_swaps: HashMap<SwapIndex, String> = HashMap::new();
    for ((ledger_id, _transfer_arg), result) in
        transfer_args.iter().zip(transfer_results.into_iter())
    {
        let swaps_for_ledger: Vec<&SwapInfo> = swaps
            .values()
            .filter(|swap| swap.tokens_amount.ledger_id == *ledger_id)
            .collect();
        for swap in swaps_for_ledger {
            match &result {
                Ok(inner_result) => match inner_result {
                    Ok(_amount) => {
                        successful_swap_ids.push(swap.index.clone());
                    }
                    Err(transfer_err) => {
                        failed_swap_ids.push(swap.index.clone());
                        failed_swaps.insert(
                            swap.index.clone(),
                            format!("Token transfer failed: {:?}", transfer_err),
                        );
                    }
                },
                Err(call_err) => {
                    failed_swap_ids.push(swap.index.clone());
                    failed_swaps.insert(
                        swap.index.clone(),
                        format!("IC call failed: {:?}", call_err),
                    );
                }
            }
        }
    }

    mutate_state(|s| {
        let swap_system = &mut s.data.swap_system;
        swap_system.update_swaps_statuses(&successful_swap_ids, SwapStatus::Reimbursed);
        swap_system.finalize_all_swaps();
        swap_system.update_swaps_statuses(
            &failed_swap_ids,
            SwapStatus::ReimburseFailed("NFT mint failed".to_string()),
        );
    });

    SwapTokensForNftResponse::Ok(swap_ids)
}
