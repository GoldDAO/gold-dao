use candid::Nat;
use gldt_swap_common::{
    gldt::{ OGYTokenSpec, GLDT_SWAP_FEE_ACCOUNT, GLDT_TX_FEE },
    swap::{
        BurnError,
        EscrowError,
        FeeTransferError,
        NftTransferError,
        RefundError,
        SwapDetailReverse,
        SwapErrorReverse,
        SwapId,
        SwapInfo,
        SwapStatus,
        SwapStatusReverse,
        TransferFailReason,
    },
};
use icrc_ledger_canister::icrc1_balance_of;
use icrc_ledger_canister_c2c_client::{ icrc1_balance_of, icrc2_transfer_from };
use icrc_ledger_types::{ icrc1::account::Account, icrc2::transfer_from::TransferFromArgs };
use origyn_nft_reference::origyn_nft_reference_canister::{
    Account as OrigynAccount,
    EscrowReceipt,
    InstantFeature,
    MarketTransferRequest,
    MarketTransferRequestReponseTxnType,
    PricingConfigShared,
    SalesConfig,
};
use origyn_nft_reference_c2c_client::market_transfer_nft_origyn;
use tracing::{ debug, error, info };
use utils::{ env::Environment, retry_async::retry_async };
use crate::{ swap::swap_info::SwapInfoTrait, utils::trace };

use crate::{ state::read_state, utils::transfer_token };

pub async fn transfer_to_escrow(swap_id: &SwapId) {
    let (swap, swap_details) = if
        let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(swap_id).cloned())
    {
        if let SwapInfo::Reverse(details) = swap_info.clone() {
            (swap_info, details)
        } else {
            debug!(
                "REVERSE SWAP :: transfer_to_escrow :: {:?} has no forward swap details",
                swap_id
            );
            return ();
        }
    } else {
        debug!("REVERSE SWAP :: transfer_to_escrow :: {:?} - can't find swap", swap_id);
        return ();
    };

    if let Err(_) = valid_for_escrow_transfer(&swap_details) {
        debug!(
            "REVERSE SWAP :: valid_for_escrow_transfer :: {:?} has the status {:?} but needs {:?}",
            swap_id,
            swap_details.status,
            SwapStatusReverse::EscrowRequest
        );
        return ();
    }
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());
    let amount =
        swap_details.tokens_to_receive.get().clone() +
        swap_details.swap_fee.clone() -
        GLDT_TX_FEE * 2;

    trace(&format!("/// amount requested {amount:?}"));
    // we request 100.8 from the user // why does the icrc2 take 2x fee? weird ( maybe one for approval and one for transfer from )
    match
        retry_async(
            ||
                icrc2_transfer_from(gldt_ledger_id, TransferFromArgs {
                    spender_subaccount: Some(swap_details.nft_id.clone().into()),
                    from: Account { owner: swap_details.user, subaccount: None },
                    to: Account {
                        owner: this_canister_id,
                        subaccount: Some(swap_details.nft_id.clone().into()),
                    },
                    amount: amount.clone(),
                    fee: None,
                    memo: None,
                    created_at_time: None,
                }),
            3
        ).await
    {
        Ok(transfer_response) => {
            match
                icrc1_balance_of(gldt_ledger_id, icrc1_balance_of::Args {
                    owner: this_canister_id,
                    subaccount: Some(swap_details.nft_id.clone().into()),
                }).await
            {
                Ok(bal) => {
                    trace(&format!("//// balance after transfer from : {bal:?}"));
                }
                Err(_) => {
                    trace("//// something went wrong getting user balance");
                } // .8
            }
            match transfer_response {
                icrc_ledger_canister::icrc2_transfer_from::Response::Ok(_) => {
                    swap.update_status(SwapStatus::Reverse(SwapStatusReverse::NftTransferRequest));
                }
                icrc_ledger_canister::icrc2_transfer_from::Response::Err(e) => {
                    swap.update_status(
                        SwapStatus::Reverse(
                            SwapStatusReverse::Failed(
                                SwapErrorReverse::EscrowFailed(
                                    EscrowError::TransferFailed(
                                        TransferFailReason::TransferFromError(e.clone())
                                    )
                                )
                            )
                        )
                    );

                    debug!(
                        "REVERSE SWAP :: escrow :: SwapId = {swap_id:?} :: error = failed to transfer to escrow with error - {e:?}"
                    );
                }
            }
        }
        Err((rejection_code, msg)) => {
            let error_message = format!("ERROR : {rejection_code:?}. message : {msg}");
            swap.update_status(
                SwapStatus::Reverse(
                    SwapStatusReverse::Failed(
                        SwapErrorReverse::EscrowFailed(
                            EscrowError::TransferFailed(
                                TransferFailReason::CallError(error_message.clone())
                            )
                        )
                    )
                )
            );

            debug!(
                "REVERSE SWAP :: escrow :: Swap Id {swap_id:?} :: error = failed to transfer to escrow with error - {error_message}"
            );
        }
    }
}

pub async fn transfer_nft(swap_id: &SwapId) {
    let (swap, swap_details) = if
        let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(swap_id).cloned())
    {
        if let SwapInfo::Reverse(details) = swap_info.clone() {
            (swap_info, details)
        } else {
            debug!("REVERSE SWAP :: transfer_nft :: {:?} has no forward swap details", swap_id);
            return ();
        }
    } else {
        debug!("REVERSE SWAP :: transfer_nft :: {:?} - can't find swap", swap_id);
        return ();
    };
    if let Err(_) = valid_for_nft_transfer(&swap_details) {
        debug!(
            "REVERSE SWAP :: valid_for_nft_transfer :: {:?} has the status {:?} but needs {:?}",
            swap_id,
            swap_details.status,
            SwapStatusReverse::NftTransferRequest
        );
        return ();
    }

    let ogy_ledger_id = read_state(|s| s.data.ogy_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());

    match
        market_transfer_nft_origyn(swap_details.nft_canister, MarketTransferRequest {
            token_id: swap_details.nft_id_string.clone(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Instant(
                    Some(
                        vec![
                            InstantFeature::FeeAccounts(
                                vec![
                                    "com.origyn.royalty.broker".to_string(),
                                    "com.origyn.royalty.node".to_string(),
                                    "com.origyn.royalty.originator".to_string(),
                                    "com.origyn.royalty.network".to_string(),
                                    "com.origyn.royalty.custom".to_string()
                                ]
                            ),
                            InstantFeature::FeeSchema("com.origyn.royalties.fixed".to_string())
                        ]
                    )
                ),
                escrow_receipt: Some(EscrowReceipt {
                    token: OGYTokenSpec::new(ogy_ledger_id).get_token_spec(),
                    token_id: swap_details.nft_id_string,
                    seller: OrigynAccount::Account { owner: this_canister_id, sub_account: None },
                    buyer: OrigynAccount::Account {
                        owner: swap_details.user,
                        sub_account: None,
                    },
                    amount: Nat::from(0u64),
                }),
            },
        }).await
    {
        Ok(res) => {
            match res {
                origyn_nft_reference::origyn_nft_reference_canister::MarketTransferResult::Ok(
                    res_ok,
                ) => {
                    trace(&format!("{res_ok:?}"));
                    info!("{res_ok:?}");
                    match res_ok.txn_type {
                        MarketTransferRequestReponseTxnType::SaleEnded { .. } => {
                            swap.update_status(SwapStatus::Reverse(SwapStatusReverse::BurnRequest));
                        }
                        other => {
                            trace(&format!("error, expected sale ended but got {other:?}"));
                            error!("error, expected sale ended but got {other:?}");
                            swap.update_status(
                                SwapStatus::Reverse(
                                    SwapStatusReverse::NftTransferFailed(
                                        NftTransferError::TransferFailed(
                                            format!("expected a sale end call but got {other:?}")
                                        )
                                    )
                                )
                            );
                        }
                    }
                }
                origyn_nft_reference::origyn_nft_reference_canister::MarketTransferResult::Err(
                    e,
                ) => {
                    trace(&format!("{e:?}"));
                    error!("{e:?}");
                    swap.update_status(
                        SwapStatus::Reverse(
                            SwapStatusReverse::NftTransferFailed(
                                NftTransferError::TransferFailed(format!("{e:?}"))
                            )
                        )
                    );
                }
            }
        }
        Err(e) => {
            error!("{e:?}");
            trace(&format!("{e:?}"));
            swap.update_status(
                SwapStatus::Reverse(
                    SwapStatusReverse::NftTransferFailed(
                        NftTransferError::CallError(format!("{e:?}"))
                    )
                )
            );
        }
    }
}

pub async fn burn_gldt(swap_id: &SwapId) {
    let (swap, swap_details) = if
        let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(swap_id).cloned())
    {
        if let SwapInfo::Reverse(details) = swap_info.clone() {
            (swap_info, details)
        } else {
            debug!("REVERSE SWAP :: burn_gldt :: {:?} has no forward swap details", swap_id);
            return ();
        }
    } else {
        debug!("REVERSE SWAP :: burn_gldt :: {:?} - can't find swap", swap_id);
        return ();
    };
    if let Err(_) = valid_for_burn(&swap_details) {
        debug!(
            "REVERSE SWAP :: valid_for_burn :: {:?} has the status {:?} but needs {:?}",
            swap_id,
            swap_details.status,
            SwapStatusReverse::BurnRequest
        );
        return ();
    }

    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());
    trace(&format!("/// will burn {} ", swap_details.tokens_to_receive.get().clone()));
    match
        retry_async(
            ||
                transfer_token(
                    swap_details.nft_id.clone().into(),
                    Account {
                        owner: this_canister_id,
                        subaccount: None,
                    },
                    gldt_ledger_id,
                    swap_details.tokens_to_receive.get().clone() // we still have 0.8 left in the pot
                ),
            3
        ).await
    {
        Ok(_) => {
            swap.update_status(SwapStatus::Reverse(SwapStatusReverse::FeeTransferRequest));
            debug!("REVERSE SWAP :: burn :: Swap Id {swap_id:?} :: success");

            match
                icrc1_balance_of(gldt_ledger_id, icrc1_balance_of::Args {
                    owner: this_canister_id,
                    subaccount: Some(swap_details.nft_id.clone().into()),
                }).await
            {
                Ok(bal) => { trace(&format!("//// balance after burn : {bal:?}")) }
                Err(_) => { trace("//// something went wrong getting user balance") } // .8
            }
        }
        Err(error_message) => {
            swap.update_status(
                SwapStatus::Reverse(
                    SwapStatusReverse::Failed(
                        SwapErrorReverse::BurnFailed(BurnError::CallError(error_message.clone()))
                    )
                )
            );
            debug!("REVERSE SWAP :: burn :: Swap Id {swap_id:?} :: error =  {error_message:?}");
        }
    }
}

pub async fn transfer_fees(swap_id: &SwapId) {
    let (swap, swap_details) = if
        let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(swap_id).cloned())
    {
        if let SwapInfo::Reverse(details) = swap_info.clone() {
            (swap_info, details)
        } else {
            debug!("REVERSE SWAP :: transfer_fees :: {:?} has no forward swap details", swap_id);
            return ();
        }
    } else {
        debug!("REVERSE SWAP :: transfer_fees :: {:?} - can't find swap", swap_id);
        return ();
    };
    if let Err(_) = valid_for_fee_transfer(&swap_details) {
        debug!(
            "REVERSE SWAP :: valid_for_fee_transfer :: {:?} has the status {:?} but needs {:?}",
            swap_id,
            swap_details.status,
            SwapStatusReverse::FeeTransferRequest
        );
        return ();
    }
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());

    match
        retry_async(
            ||
                transfer_token(
                    swap_details.nft_id.clone().into(),
                    Account {
                        owner: this_canister_id,
                        subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
                    },
                    gldt_ledger_id,
                    swap_details.swap_fee.clone() - Nat::from(GLDT_TX_FEE * 3) // at this point, there is .8 in swap lefts left
                ),
            3
        ).await
    {
        Ok(_) => {
            swap.update_status(SwapStatus::Reverse(SwapStatusReverse::Complete));
            debug!("REVERSE SWAP :: fee transfer :: Swap Id {swap_id:?} :: success");
            match
                icrc1_balance_of(gldt_ledger_id, icrc1_balance_of::Args {
                    owner: this_canister_id,
                    subaccount: Some(swap_details.nft_id.clone().into()),
                }).await
            {
                Ok(bal) => { trace(&format!("//// balance after transferring fees : {bal:?}")) }
                Err(_) => { trace("//// something went wrong getting user balance") } // .8
            }
        }
        Err(error_message) => {
            swap.update_status(
                SwapStatus::Reverse(
                    SwapStatusReverse::Failed(
                        SwapErrorReverse::FeeTransferFailed(
                            FeeTransferError::CallError(error_message.clone())
                        )
                    )
                )
            );
            debug!("REVERSE SWAP :: burn :: Swap Id {swap_id:?} :: error =  {error_message:?}");
        }
    }
}

pub async fn refund(swap_id: &SwapId) {
    let (swap, swap_details) = if
        let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(swap_id).cloned())
    {
        if let SwapInfo::Reverse(details) = swap_info.clone() {
            (swap_info, details)
        } else {
            debug!("REVERSE SWAP :: refund :: {:?} has no forward swap details", swap_id);
            return ();
        }
    } else {
        debug!("REVERSE SWAP :: refund :: {:?} - can't find swap", swap_id);
        return ();
    };
    let swap_status_reverse = match valid_for_refund(&swap_details) {
        Ok(status) => status,
        Err(_) => {
            debug!(
                "REVERSE SWAP :: valid_for_refund :: {:?} has the status {:?} but needs {}",
                swap_id,
                swap_details.status,
                "SwapStatusReverse::NftTransferFailed(_)"
            );
            return ();
        }
    };
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);

    match
        retry_async(
            ||
                transfer_token(
                    swap_details.nft_id.clone().into(),
                    Account {
                        owner: swap_details.user,
                        subaccount: None,
                    },
                    gldt_ledger_id,
                    swap_details.tokens_to_receive.get().clone() +
                        swap_details.swap_fee.clone() -
                        swap_details.transfer_fees.clone() -
                        GLDT_TX_FEE
                ),
            3
        ).await
    {
        Ok(_) => {
            swap.update_status(
                SwapStatus::Reverse(
                    SwapStatusReverse::Failed(
                        SwapErrorReverse::Refunded(Box::new(swap_status_reverse.clone()))
                    )
                )
            );
            debug!("REVERSE SWAP :: refund :: Swap Id {swap_id:?} :: success");
        }
        Err(msg) => {
            swap.update_status(
                SwapStatus::Reverse(
                    SwapStatusReverse::RefundFailed(RefundError::CallError(msg.clone()))
                )
            );
            debug!("REVERSE SWAP :: burn :: Swap Id {swap_id:?} :: error =  {msg:?}");
        }
    }
}

fn valid_for_refund(current_swap_details: &SwapDetailReverse) -> Result<&SwapStatusReverse, ()> {
    let current_swap_status = &current_swap_details.status;
    if !matches!(current_swap_status, &SwapStatusReverse::NftTransferFailed(_)) {
        Err(())
    } else {
        Ok(current_swap_status)
    }
}

fn valid_for_fee_transfer(current_swap_details: &SwapDetailReverse) -> Result<(), ()> {
    let current_swap_status = &current_swap_details.status;
    if current_swap_status != &SwapStatusReverse::FeeTransferRequest {
        Err(())
    } else {
        Ok(())
    }
}

fn valid_for_burn(current_swap_details: &SwapDetailReverse) -> Result<(), ()> {
    let current_swap_status = &current_swap_details.status;
    if current_swap_status != &SwapStatusReverse::BurnRequest {
        Err(())
    } else {
        Ok(())
    }
}

fn valid_for_nft_transfer(current_swap_details: &SwapDetailReverse) -> Result<(), ()> {
    let current_swap_status = &current_swap_details.status;
    if current_swap_status != &SwapStatusReverse::NftTransferRequest {
        Err(())
    } else {
        Ok(())
    }
}

fn valid_for_escrow_transfer(current_swap_details: &SwapDetailReverse) -> Result<(), ()> {
    let current_swap_status = &current_swap_details.status;
    if current_swap_status != &SwapStatusReverse::EscrowRequest {
        Err(())
    } else {
        Ok(())
    }
}
