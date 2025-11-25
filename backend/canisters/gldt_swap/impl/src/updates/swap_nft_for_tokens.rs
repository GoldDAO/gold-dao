use crate::guards::{reject_anonymous_caller, GuardNft};
use crate::model::nft_batches::GroupedNftsByCanister;
use crate::state::{icrc3_commit_prepared_transaction, mutate_state, read_state};
use crate::utils::prepare_transactions;
use bity_ic_canister_time::SECOND_IN_MS;
use candid::Principal;
use gldt_swap_api_canister::swap_nft_for_tokens::SwapNftForTokensErrors;
pub use gldt_swap_api_canister::swap_nft_for_tokens::{
    Args as SwapNftForTokensArgs, Response as SwapNftForTokensResponse,
};
use gldt_swap_api_canister::swap_tokens_for_nft::RetryInMilliseconds;
use gldt_swap_common::general_error::GeneralError;
use gldt_swap_common::swap::{SwapIndex, SwapInfo, SwapStatus, SwapType};
use ic_cdk::api::msg_caller;
use ic_cdk::update;
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use std::collections::HashMap;
use tracing::{error, info};

#[update]
async fn swap_nft_for_tokens(args: SwapNftForTokensArgs) -> SwapNftForTokensResponse {
    swap_nft_for_tokens_impl(args).await
}

pub async fn swap_nft_for_tokens_impl(args: SwapNftForTokensArgs) -> SwapNftForTokensResponse {
    let _span = tracing::info_span!("SWAP_NFT_FOR_TOKENS").entered();

    let caller = msg_caller();

    reject_anonymous_caller()
        .map_err(|e| SwapNftForTokensErrors::GeneralError(GeneralError::InvalidPrincipal(e)))?;

    if args.is_empty() {
        return Err(SwapNftForTokensErrors::GeneralError(
            GeneralError::EmptyArgs("There were no NFTs provided for swap".to_string()),
        ));
    }

    if args.len() > 100 {
        return Err(SwapNftForTokensErrors::Limit(
            "You may only swap 100 in any given request. batch your calls in batches of 100"
                .to_string(),
        ));
    }

    let is_balancer_running = read_state(|s| s.data.is_gldt_supply_balancer_running);
    if is_balancer_running {
        return Err(SwapNftForTokensErrors::Retry(RetryInMilliseconds(
            SECOND_IN_MS * 30,
            "the supply is currently being balanced. please try again in 15 seconds".to_string(),
        )));
    }

    read_state(|s| s.data.swap_configs.validate_nfts(&args))?;

    let batched_nfts = GroupedNftsByCanister::from_nfts(args.clone());

    batched_nfts
        .validate_all_owned_by(caller)
        .await
        .into_iter()
        .find_map(|res| res.err())
        .map_or(Ok(()), |err| {
            Err(SwapNftForTokensErrors::GeneralError(
                GeneralError::UserIsNotNftOwner(err.to_string()),
            ))
        })?;

    let _nft_guards: Vec<_> = args
        .iter()
        .map(|nft| {
            GuardNft::new(nft.clone()).map_err(|e| {
                error!("NFT guard creation failed: {:?}", e);
                SwapNftForTokensErrors::GeneralError(GeneralError::AlreadyProcessing(e))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    // 1. Create swaps
    let swaps = mutate_state(|s| {
        s.data.swap_system.create_swaps_batch(
            SwapType::Forward,
            caller,
            args.clone(),
            &s.data.swap_configs,
        )
    })
    .map_err(|e| {
        error!("Failed to create swaps: {:?}", e);
        SwapNftForTokensErrors::GeneralError(e)
    })?;

    let swap_ids: Vec<candid::Nat> = swaps.keys().cloned().collect();

    let prepared_transactions = prepare_transactions(&swaps)?;
    ic_cdk::println!("prepared_transactions {:?}", prepared_transactions);

    // 2. Transfer NFTs
    let transfer_results = batched_nfts.batch_transfer_from_all(caller).await;

    let mut successful_nfts = Vec::new();
    let mut successful_swap_ids = Vec::new();
    let mut failed_swaps: HashMap<SwapIndex, String> = HashMap::new();

    for swap in swaps.values() {
        if let Some(results) = transfer_results.get(&swap.nft.canister_id) {
            let nft_success: Result<bool, _> =
                results.iter().try_fold(false, |found, res| match res {
                    Ok(nft_id) if *nft_id == swap.nft.id => Ok(true),
                    Ok(_) => Ok(found),
                    Err(e) => Err(e),
                });

            match nft_success {
                Ok(true) => {
                    successful_nfts.push(swap.nft.clone());
                    successful_swap_ids.push(swap.index.clone());
                }
                Ok(false) => {
                    failed_swaps.insert(
                        swap.index.clone(),
                        "NFT not found in transfer results".to_string(),
                    );
                }
                Err(err) => {
                    failed_swaps.insert(swap.index.clone(), format!("Transfer error: {:?}", err));
                }
            }
        } else {
            failed_swaps.insert(
                swap.index.clone(),
                "No transfer attempt for this canister".to_string(),
            );
        }
    }

    mutate_state(|s| {
        let swap_system = &mut s.data.swap_system;

        swap_system.update_swaps_statuses(&successful_swap_ids, SwapStatus::NftTransferredFrom);

        for (swap_index, error_msg) in &failed_swaps {
            swap_system.update_swaps_status(
                swap_index,
                SwapStatus::NftTransferFromFailed(error_msg.clone()),
            );
        }
    });

    if successful_swap_ids.is_empty() {
        return Err(SwapNftForTokensErrors::GeneralError(
            GeneralError::CallError(
                "No NFT transfers succeeded; tokens are not transferred.".into(),
            ),
        ));
    }

    let filtered_nfts = GroupedNftsByCanister::from_nfts(successful_nfts);

    // 3. Transfer tokens
    let tokens_amount: HashMap<Principal, candid::Nat> =
        filtered_nfts.calculate_token_equivalent().map_err(|e| {
            error!("Token calculation failed: {:?}", e);
            SwapNftForTokensErrors::GeneralError(e)
        })?;

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
        swap_system.update_swaps_statuses(&successful_swap_ids, SwapStatus::Minted);
        swap_system.update_swaps_statuses(
            &failed_swap_ids,
            SwapStatus::MintFailed("NFT mint failed".to_string()),
        );
    });

    mutate_state(|s| {
        s.data
            .swap_system
            .update_swaps_statuses(&swap_ids, SwapStatus::Complete);
    });

    info!("All token transfers completed");
    let complete_swaps = read_state(|s| s.data.swap_system.filter_completed_swaps(&swap_ids));

    for (swap_index, _) in complete_swaps {
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

    let grouped_to_transfer = GroupedNftsByCanister::from_nfts(nfts_for_reimburse.clone());
    let nft_transfer_results = grouped_to_transfer.batch_transfer_all(caller).await;

    let mut nft_transfer_success: Vec<SwapIndex> = Vec::new();
    let mut nft_transfer_failed: HashMap<SwapIndex, String> = HashMap::new();

    for swap in swaps_to_reimburse.values() {
        if let Some(results) = nft_transfer_results.get(&swap.nft.canister_id) {
            let nft_success = results
                .iter()
                .find_map(|res| match res {
                    Ok(nft_id) if *nft_id == swap.nft.id => Some(Ok(true)),
                    Err(e) => Some(Err(e)),
                    _ => None,
                })
                .unwrap_or(Ok(false));

            match nft_success {
                Ok(true) => {
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
                        .insert(swap.index.clone(), format!("NFT transfer error: {:?}", err));
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
        let swap_system = &mut s.data.swap_system;
        swap_system.update_swaps_statuses(&nft_transfer_success, SwapStatus::Reimbursed);
        swap_system.finalize_all_swaps();
        for (idx, reason) in &nft_transfer_failed {
            swap_system.update_swaps_status(idx, SwapStatus::ReimburseFailed(reason.clone()));
        }
    });

    SwapNftForTokensResponse::Ok(swap_ids)
}
