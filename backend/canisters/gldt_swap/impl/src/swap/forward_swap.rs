use std::{ array::TryFromSliceError, ops::Deref };

use candid::{ Nat, Principal };
use canister_time::timestamp_nanos;
use gldt_swap_common::{
    gldt::{ GldtNumTokens, GldtTokenSpec, GLDT_LEDGER_FEE_ACCOUNT, GLDT_TX_FEE, MEMO_GLDT_SWAP },
    swap::{
        BidFailError,
        BurnFeesError,
        DepositRecoveryError,
        MintError,
        NotificationError,
        SwapDetailForward,
        SwapErrorForward,
        SwapId,
        SwapInfo,
        SwapStatus,
        SwapStatusForward,
        TransferFailReason,
    },
};
use ic_stable_structures::Storable;
use origyn_nft_reference::origyn_nft_reference_canister::{
    Account as NftSellerAccount,
    AskFeature,
    BidFeature,
    BidRequest,
    BidResponseTxnType,
    DepositDetail,
    EndingType,
    EscrowRecord,
    EscrowRequest,
    IcTokenSpec,
    ManageSaleRequest,
    ManageSaleResponse,
    ManageSaleResult,
    PricingConfigShared,
    SaleStatusSharedSaleType,
    TokenSpec,
    WithdrawDescription,
    WithdrawRequest,
};
pub use gldt_swap_api_canister::notify_sale_nft_origyn::Args as SubscriberNotification;
use origyn_nft_reference_c2c_client::sale_nft_origyn;
use origyn_nft_reference::origyn_nft_reference_canister::{ Account as OrigynAccount };
use tracing::{ debug, info };
use utils::{ env::Environment, retry_async::retry_async };
use icrc_ledger_types::icrc1::{
    account::{ Account, Subaccount },
    transfer::{ Memo as MemoIcrc, TransferArg },
};
use serde_bytes::ByteBuf;
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use crate::{ state::{ mutate_state, read_state }, utils::transfer_token };
use crate::swap::swap_info::SwapInfoTrait;

pub fn forward_swap_validate_notification(swap_id: &SwapId, notification: &SubscriberNotification) {
    let (swap, swap_details) = if
        let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(swap_id).cloned())
    {
        if let SwapInfo::Forward(details) = swap_info.clone() {
            (swap_info, details)
        } else {
            debug!(
                "FORWARD SWAP :: forward_swap_validate_notification :: {:?} has no forward swap details",
                swap_id
            );
            return;
        }
    } else {
        debug!(
            "FORWARD SWAP :: forward_swap_validate_notification :: {:?} - can't find swap",
            swap_id
        );
        return;
    };

    if let Err(_) = valid_init_state(&swap_details) {
        debug!(
            "FORWARD SWAP :: valid_init_state :: {:?} has the status {:?} but needs {:?}",
            swap_id,
            swap_details.status,
            SwapStatusForward::Init
        );
        return;
    }
    swap.update_status(SwapStatus::Forward(SwapStatusForward::NotificationInProgress));

    if swap_details.nft_id_string != notification.sale.token_id {
        let swap_nft_id_string = swap_details.nft_id_string;
        let noti_nft_id = notification.sale.token_id.clone();
        set_notification_error(
            &swap,
            NotificationError::OrigynStringIdDoesNotMatch(
                format!(
                    "swap has nft_id of : {swap_nft_id_string}. notification has nft id of  : {noti_nft_id}"
                )
            )
        );
        return;
    }

    if swap_details.nft_canister != notification.collection {
        let swap_coll = swap_details.nft_canister;
        let noti_coll = notification.collection;
        set_notification_error(
            &swap,
            NotificationError::CollectionDoesNotMatch(
                format!(
                    "swap has collection : {swap_coll}. notification has collection : {noti_coll}"
                )
            )
        );
        return;
    }

    if let Err(e) = is_valid_seller(notification.seller.clone(), swap_details.gldt_receiver.owner) {
        set_notification_error(&swap, e);
        return; // Return early if there is an error
    }

    let escrow_sub_account: Subaccount = match validate_nft_escrow_subaccount(&notification) {
        Ok(sub_account) => sub_account,
        Err(e) => {
            set_notification_error(&swap, e);
            return ();
        }
    };

    let (token, config) = match verify_token_spec(&notification) {
        Ok(toke_and_config) => toke_and_config,
        Err(e) => {
            set_notification_error(&swap, e);
            return ();
        }
    };

    if let Err(e) = validate_sale_config(&config, &swap_details.tokens_to_mint, &token) {
        set_notification_error(&swap, e);
        return ();
    }

    if let Err(e) = validate_sale_id_length(&notification.sale.sale_id) {
        set_notification_error(&swap, e);
        return ();
    }

    mutate_state(|s| {
        if let Some(SwapInfo::Forward(details)) = s.data.swaps.get_active_swap_mut(&swap_id) {
            details.update_escrow_account(escrow_sub_account);
            details.update_sale_id(notification.sale.sale_id.clone());
        }
    });
    read_state(|s| {
        if let Some(swa) = s.data.swaps.get_active_swap(&swap_id.clone()) {
            match swa {
                SwapInfo::Forward(deets) => {
                    let i = format!(
                        "FORWARD SWAP :: forward_swap_validate_notification :: escrow account and sale id updated for {:?}",
                        deets
                    );
                    info!("{i}");
                }
                SwapInfo::Reverse(_) => {}
            }
        }
    });

    swap.update_status(SwapStatus::Forward(SwapStatusForward::MintRequest));
}

fn is_valid_seller(
    account: NftSellerAccount,
    swap_seller: Principal
) -> Result<(), NotificationError> {
    match account {
        NftSellerAccount::Account { owner, .. } => {
            if owner == swap_seller {
                return Ok(());
            }
            return Err(
                NotificationError::SellerAndReceiverDoesNotMatch(
                    format!(
                        "swap has seller of : {swap_seller}. notification has seller of : owner : {owner}"
                    )
                )
            );
        }
        NftSellerAccount::Principal_(owner) => {
            if owner == swap_seller {
                return Ok(());
            }
            return Err(
                NotificationError::SellerAndReceiverDoesNotMatch(
                    format!(
                        "swap has seller of : {swap_seller}. notification has seller of : owner : {owner}"
                    )
                )
            );
        }
        _ => {
            Err(
                NotificationError::SellerIsNotPrincipalOrAccount(
                    format!("seller in notification is not a Principal type")
                )
            )
        }
    }
}

fn set_notification_error(swap_info: &SwapInfo, error: NotificationError) {
    let swap_id = &swap_info.get_swap_id();
    let mut error_message = format!(
        "FORWARD SWAP :: notification validation  :: SwapID = {swap_id:?} :: "
    );
    match &error {
        NotificationError::OrigynStringIdDoesNotMatch(msg) => {
            error_message.push_str(&msg.clone());
        }

        NotificationError::CollectionDoesNotMatch(msg) => {
            error_message.push_str(&msg.clone());
        }
        NotificationError::SellerAndReceiverDoesNotMatch(msg) => {
            error_message.push_str(&msg.clone());
        }
        NotificationError::InvalidEscrowSubaccount(msg) => {
            error_message.push_str(&msg.clone());
        }
        NotificationError::TimeoutInvalid(msg) => {
            error_message.push_str(&msg.clone());
        }
        NotificationError::InvalidTokenSpec => {
            error_message.push_str("token spec is not correct");
        }
        NotificationError::InvalidTokenAmount => {
            error_message.push_str("notification does not contain correct token amount");
        }
        NotificationError::InvalidSaleSubaccount => {
            error_message.push_str("sale subaccount is invalid");
        }
        NotificationError::SellerIsNotPrincipalOrAccount(msg) => {
            error_message.push_str(&msg.clone());
        }
        NotificationError::TooManyPrincipalsInAllowList => {
            error_message.push_str(
                "There should only be 1 principal in the allowed list. The only allowed principal is the swap canister"
            );
        }
        NotificationError::AllowListDoesNotContainCorrectPrincipal => {
            error_message.push_str("AllowedList should contain the swap canister principal");
        }
        NotificationError::InvalidCustomAskFeature => {
            error_message.push_str("You can't supply custom AskFeatures");
        }
        NotificationError::InvalidPricingConfig => {
            error_message.push_str("Pricing config is invalid");
        }
        NotificationError::SaleIDStringTooLong(e) => {
            error_message.push_str(&e.clone());
        }
    }

    swap_info.update_status(
        SwapStatus::Forward(SwapStatusForward::Failed(SwapErrorForward::NotificationFailed(error)))
    );
    debug!("{error_message}");
}

fn validate_nft_escrow_subaccount(
    args: &SubscriberNotification
) -> Result<Subaccount, NotificationError> {
    if args.escrow_info.account.sub_account.as_slice().len() != 32 {
        return Err(
            NotificationError::InvalidEscrowSubaccount(
                format!(
                    "Escrow sub accoun (escrow_info.account.sub_account) is not of the correct length. can't convert to a Subaccount type"
                )
            )
        );
    }
    let b: Result<[u8; 32], TryFromSliceError> = args.escrow_info.account.sub_account
        .as_slice()
        .try_into();
    match b {
        Ok(x) => Ok(x),
        Err(e) => { Err(NotificationError::InvalidEscrowSubaccount(format!("{e}"))) }
    }
}

fn validate_sale_id_length(sale_id: &String) -> Result<bool, NotificationError> {
    let len = sale_id.len();
    if len > 250 {
        return Err(
            NotificationError::SaleIDStringTooLong(
                format!("The length of the sale id may not pass 1000. it's current length is {len}")
            )
        );
    }
    Ok(true)
}

fn validate_sale_config(
    config: &PricingConfigShared,
    tokens_to_mint: &GldtNumTokens,
    token_spec: &IcTokenSpec
) -> Result<(), NotificationError> {
    match config {
        PricingConfigShared::Ask(Some(features)) => {
            for feature in features {
                match feature {
                    AskFeature::BuyNow(val) => {
                        if val.clone() != tokens_to_mint.get_with_fee() {
                            return Err(NotificationError::InvalidTokenAmount);
                        }
                    }
                    AskFeature::Token(val) => {
                        match val {
                            TokenSpec::Ic(ts) => {
                                if ts != token_spec {
                                    return Err(NotificationError::InvalidTokenSpec);
                                }
                            }
                            TokenSpec::Extensible(_) => {
                                return Err(NotificationError::InvalidTokenSpec);
                            }
                        }
                    }
                    AskFeature::AllowList(principals) => {
                        if principals.len() > 1 {
                            return Err(NotificationError::TooManyPrincipalsInAllowList);
                        }
                        let this_canister_id = read_state(|s| s.env.canister_id());
                        if !principals.contains(&this_canister_id) {
                            return Err(NotificationError::AllowListDoesNotContainCorrectPrincipal);
                        }
                    }
                    AskFeature::Ending(EndingType::Timeout(timeout_value)) => {
                        if timeout_value.clone() != Nat::from(180_000_000_000u64) {
                            return Err(
                                NotificationError::TimeoutInvalid(
                                    format!(
                                        "Timeout is {timeout_value} but should be 180_000_000_000"
                                    )
                                )
                            );
                        }
                    }
                    | AskFeature::Kyc(_)
                    | AskFeature::Notify(_)
                    | AskFeature::FeeSchema(_)
                    | AskFeature::FeeAccounts(_) => {}
                    _ => {
                        return Err(NotificationError::InvalidCustomAskFeature);
                    }
                }
            }
        }
        _ => {
            return Err(NotificationError::InvalidPricingConfig);
        }
    }
    Ok(())
}

fn verify_token_spec(
    args: &SubscriberNotification
) -> Result<(IcTokenSpec, PricingConfigShared), NotificationError> {
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let (token, config) = match &args.sale.sale_type {
        SaleStatusSharedSaleType::Auction(t) => { (t.token.clone(), t.config.clone()) }
    };

    if let TokenSpec::Ic(ic_token_spec) = token {
        let token_spec = GldtTokenSpec::new(gldt_ledger_id).get_ic_token_spec();
        if ic_token_spec != token_spec {
            return Err(NotificationError::InvalidTokenSpec);
        }
        Ok((ic_token_spec, config))
    } else {
        Err(NotificationError::InvalidTokenSpec)
    }
}

pub async fn forward_swap_perform_mint_to_escrow(swap_id: &SwapId) {
    let (swap, swap_details) = if
        let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(swap_id).cloned())
    {
        if let SwapInfo::Forward(details) = swap_info.clone() {
            (swap_info, details)
        } else {
            debug!(
                "FORWARD SWAP :: forward_swap_perform_mint_to_escrow :: {:?} has no forward swap details",
                swap_id
            );
            return ();
        }
    } else {
        debug!(
            "FORWARD SWAP :: forward_swap_perform_mint_to_escrow :: {:?} - can't find swap",
            swap_id
        );
        return ();
    };
    let nft_id = swap_id.0.clone();
    if let Err(_) = valid_for_mint(&swap_details) {
        debug!(
            "FORWARD SWAP :: valid_for_mint :: {:?} has the status {:?} but needs {:?}",
            swap_id,
            swap_details.status,
            SwapStatusForward::MintRequest
        );
        return ();
    }

    swap.update_status(SwapStatus::Forward(SwapStatusForward::MintInProgress));

    let gldt_canister_id = read_state(|s| s.data.gldt_ledger_id);

    let args = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: swap_details.nft_canister,
            subaccount: Some(swap_details.escrow_sub_account),
        },
        amount: swap_details.tokens_to_mint.get_with_fee().clone(),
        fee: None,
        created_at_time: Some(timestamp_nanos()),
        memo: Some(MemoIcrc(ByteBuf::from(swap.get_swap_id().1.to_string()))),
    };

    match retry_async(|| icrc1_transfer(gldt_canister_id, &args), 3).await {
        Ok(Ok(_)) => {
            swap.update_status(SwapStatus::Forward(SwapStatusForward::BidRequest));
        }

        Ok(Err(msg)) => {
            debug!("FORWARD SWAP :: NFT ID = {nft_id:?} :: mint :: error :: {msg}");

            swap.update_status(
                SwapStatus::Forward(
                    SwapStatusForward::Failed(
                        SwapErrorForward::MintFailed(
                            MintError::TransferFailed(
                                TransferFailReason::TransferError(msg.clone())
                            )
                        )
                    )
                )
            );
        }
        Err((_, msg)) => {
            debug!("FORWARD SWAP :: NFT ID = {nft_id:?} :: mint :: error :: {msg}");
            swap.update_status(
                SwapStatus::Forward(
                    SwapStatusForward::Failed(
                        SwapErrorForward::MintFailed(
                            MintError::TransferFailed(TransferFailReason::CallError(msg.clone()))
                        )
                    )
                )
            );
        }
    }
}

pub async fn forward_swap_perform_bid_on_nft(
    swap_id: &SwapId,
    notification: SubscriberNotification
) {
    info!("FORWARD SWAP :: forward_swap_perform_bid_on_nft :: entered ");
    let (swap, swap_details) = if
        let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(swap_id).cloned())
    {
        if let SwapInfo::Forward(details) = swap_info.clone() {
            (swap_info, details)
        } else {
            debug!(
                "FORWARD SWAP :: forward_swap_perform_bid_on_nft :: {:?} has no forward swap details",
                swap_id
            );
            return ();
        }
    } else {
        debug!(
            "FORWARD SWAP :: forward_swap_perform_bid_on_nft :: {:?} - can't find swap",
            swap_id
        );
        return ();
    };
    if let Err(_) = valid_for_bid(&swap_details) {
        debug!(
            "FORWARD SWAP :: valid_for_bid :: {:?} has the status {:?} but needs {:?}",
            swap_id,
            swap_details.status,
            SwapStatusForward::BidRequest
        );
        return ();
    }

    swap.update_status(SwapStatus::Forward(SwapStatusForward::BidInProgress));
    let nft_id = swap_id.0.clone();
    let nft_canister_id = swap_details.nft_canister;
    let gldt_ledger_canister_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());
    let token_spec = GldtTokenSpec::new(gldt_ledger_canister_id).get_ic_token_spec();
    let bid = BidRequest {
        config: Some(
            vec![
                BidFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                BidFeature::FeeAccounts(
                    vec![
                        "com.origyn.royalty.node".to_string(),
                        "com.origyn.royalty.broker".to_string(),
                        "com.origyn.royalty.originator".to_string(),
                        "com.origyn.royalty.custom".to_string(),
                        "com.origyn.royalty.network".to_string()
                    ]
                )
            ]
        ),
        escrow_record: EscrowRecord {
            token: origyn_nft_reference::origyn_nft_reference_canister::TokenSpec::Ic(token_spec),
            token_id: swap_details.nft_id_string,
            seller: notification.seller,
            lock_to_date: None,
            buyer: NftSellerAccount::Principal_(this_canister_id),
            amount: swap_details.tokens_to_mint.get_with_fee().clone(),
            sale_id: Some(swap_details.sale_id.clone()),
            account_hash: None,
        },
    };

    match
        retry_async(
            || sale_nft_origyn(nft_canister_id, ManageSaleRequest::Bid(bid.clone())),
            3
        ).await
    {
        Ok(sale_result) => {
            match sale_result {
                ManageSaleResult::Ok(val) => {
                    if let ManageSaleResponse::Bid(bid) = *val {
                        match bid.txn_type {
                            BidResponseTxnType::SaleEnded { .. } => {
                                swap.update_status(
                                    SwapStatus::Forward(SwapStatusForward::BurnFeesRequest)
                                );
                            }
                            _ => {
                                debug!(
                                    "FORWARD SWAP :: bid :: NFT ID = {nft_id:?} :: error :: sale has not ended :: bid args = {bid:?}"
                                );
                                swap.update_status(
                                    SwapStatus::Forward(
                                        SwapStatusForward::BidFail(
                                            BidFailError::UnexpectedError(
                                                "Sale not ended".to_string()
                                            )
                                        )
                                    )
                                );
                            }
                        };
                    } else {
                        debug!(
                            "FORWARD SWAP :: bid :: NFT ID = {nft_id:?} :: error :: result is not a bid :: bid args = {bid:?}"
                        );
                        swap.update_status(
                            SwapStatus::Forward(
                                SwapStatusForward::BidFail(
                                    BidFailError::UnexpectedError("Sale not ended".to_string())
                                )
                            )
                        );
                    }
                }
                ManageSaleResult::Err(e) => {
                    debug!(
                        "FORWARD SWAP :: bid :: NFT ID = {nft_id:?} :: error = {e:?} :: bid args = {bid:?}"
                    );

                    swap.update_status(
                        SwapStatus::Forward(
                            SwapStatusForward::BidFail(
                                BidFailError::TransferFailed(format!("{e:?}"))
                            )
                        )
                    );
                }
            }
        }
        Err(e) => {
            debug!("FORWARD SWAP :: bid :: NFT ID = {nft_id:?} :: error :: {e:?}");

            swap.update_status(
                SwapStatus::Forward(
                    SwapStatusForward::BidFail(BidFailError::CallError(format!("{e:?}")))
                )
            );
        }
    }
}

pub async fn forward_swap_perform_burn_fees(swap_id: &SwapId) {
    let (swap, swap_details) = if
        let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(swap_id).cloned())
    {
        if let SwapInfo::Forward(details) = swap_info.clone() {
            (swap_info, details)
        } else {
            debug!(
                "FORWARD SWAP :: forward_swap_perform_mint_to_escrow :: {:?} has no forward swap details",
                swap_id
            );
            return ();
        }
    } else {
        debug!(
            "FORWARD SWAP :: forward_swap_perform_mint_to_escrow :: {:?} - can't find swap",
            swap_id
        );
        return ();
    };

    if let Err(_) = valid_for_burn_fees(&swap_details) {
        debug!(
            "FORWARD SWAP :: valid_for_burn_fees :: {:?} has the status {:?} but needs {:?}",
            swap_id,
            swap_details.status,
            SwapStatusForward::BurnFeesRequest
        );
        return ();
    }
    swap.update_status(SwapStatus::Forward(SwapStatusForward::BurnFeesInProgress));
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());
    match
        retry_async(
            ||
                transfer_token(
                    GLDT_LEDGER_FEE_ACCOUNT,
                    Account {
                        owner: this_canister_id, // burn account
                        subaccount: None,
                    },
                    gldt_ledger_id,
                    Nat::from(GLDT_TX_FEE * 2),
                    Some(MemoIcrc(ByteBuf::from(swap.get_swap_id().1.0.to_bytes_le())))
                ),
            3
        ).await
    {
        Ok(_) => {
            swap.update_status(SwapStatus::Forward(SwapStatusForward::Complete));
        }
        Err(e) => {
            debug!("FORWARD SWAP :: perform_burn_fees :: SwapId = {swap_id:?} :: error :: {e:?}");
            swap.update_status(
                SwapStatus::Forward(
                    SwapStatusForward::BurnFeesFailed(
                        BurnFeesError::TransferFailed(TransferFailReason::CallError(e))
                    )
                )
            );
        }
    }

    return ();
}

pub async fn forward_swap_perform_deposit_recovery(swap_id: &SwapId) -> Result<(), ()> {
    let (swap, swap_details) = if
        let Some(swap_info) = read_state(|s| s.data.swaps.get_active_swap(swap_id).cloned())
    {
        if let SwapInfo::Forward(details) = swap_info.clone() {
            (swap_info, details)
        } else {
            debug!(
                "FORWARD SWAP :: forward_swap_perform_deposit_recovery :: {:?} has no forward swap details",
                swap_id
            );
            return Err(());
        }
    } else {
        debug!(
            "FORWARD SWAP :: forward_swap_perform_deposit_recovery :: {:?} - can't find swap",
            swap_id
        );
        return Err(());
    };

    let original_error = match swap_details.status {
        SwapStatusForward::DepositRecoveryRequest(swap_status_forward) => {
            swap_status_forward.deref().clone()
        }
        _ => {
            debug!(
                "FORWARD SWAP :: forward_swap_perform_deposit_recovery :: {:?} has the status {:?}",
                swap_id,
                swap_details.status
            );
            return Err(());
        }
    };
    swap.update_status(
        SwapStatus::Forward(
            SwapStatusForward::DepositRecoveryInProgress(Box::new(original_error.clone()))
        )
    );
    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());

    let args = ManageSaleRequest::RecognizeEscrow(EscrowRequest {
        token_id: swap_details.nft_id_string.clone(),
        deposit: DepositDetail {
            token: GldtTokenSpec::new(gldt_ledger_id).get_token_spec(),
            trx_id: None,
            seller: OrigynAccount::Account {
                owner: swap_details.gldt_receiver.owner,
                sub_account: None,
            },
            buyer: OrigynAccount::Account {
                owner: this_canister_id,
                sub_account: None,
            },
            amount: swap_details.tokens_to_mint.get_with_fee().clone(),
            sale_id: Some(swap_details.sale_id.clone()),
        },
        lock_to_date: None,
    });

    let valid_deposit = match sale_nft_origyn(swap_details.nft_canister, args).await {
        Ok(_deposit_result) => {
            match _deposit_result {
                ManageSaleResult::Ok(_) => {
                    true // deposit recognized
                }
                ManageSaleResult::Err(origyn_error) => {
                    swap.update_status(
                        SwapStatus::Forward(
                            SwapStatusForward::DepositRecoveryFailed(
                                Box::new(original_error.clone()),
                                DepositRecoveryError::CantRecover(format!("{origyn_error:?}"))
                            )
                        )
                    );
                    false
                }
            }
        }
        Err(e) => {
            swap.update_status(
                SwapStatus::Forward(
                    SwapStatusForward::DepositRecoveryFailed(
                        Box::new(original_error.clone()),
                        DepositRecoveryError::CallError(format!("{e:?}"))
                    )
                )
            );
            false
        }
    };

    if !valid_deposit {
        return Err(());
    }

    let args = ManageSaleRequest::Withdraw(
        WithdrawRequest::Escrow(WithdrawDescription {
            token: GldtTokenSpec::new(gldt_ledger_id).get_token_spec_with_no_fee(),
            token_id: swap_details.nft_id_string.clone(),
            seller: OrigynAccount::Account {
                owner: swap_details.gldt_receiver.owner,
                sub_account: None,
            },
            withdraw_to: OrigynAccount::Principal_(this_canister_id),
            buyer: OrigynAccount::Account {
                owner: this_canister_id,
                sub_account: None,
            },
            amount: swap_details.tokens_to_mint.get_with_fee(),
        })
    );

    let manage_sale_result = sale_nft_origyn(swap_details.nft_canister, args).await;

    match manage_sale_result {
        Ok(ManageSaleResult::Ok(_)) => {
            match &original_error {
                SwapStatusForward::BidRequest => {
                    let _ = &swap.update_status(
                        // we have the deposit back, a request must have timed out or the notification never came to the swap canister so there should be no error to propogate
                        SwapStatus::Forward(SwapStatusForward::Failed(SwapErrorForward::Expired))
                    );
                    return Ok(());
                }
                SwapStatusForward::BidFail(e) => {
                    // fail it like normal
                    let _ = &swap.update_status(
                        SwapStatus::Forward(
                            SwapStatusForward::Failed(SwapErrorForward::BidFailed(e.clone()))
                        )
                    );
                    return Ok(());
                }

                other_status => {
                    let swap_id = swap.get_swap_id();
                    swap.update_status(
                        SwapStatus::Forward(
                            SwapStatusForward::DepositRecoveryFailed(
                                Box::new(original_error.clone()),
                                DepositRecoveryError::CantRecover(
                                    format!(
                                        "SwapId : {swap_id:?} Tried to recover a swap deposit that wasn't previously a BidRequest or BidFail. Current status : {other_status:?}"
                                    )
                                )
                            )
                        )
                    );
                    return Err(());
                }
            }
        }
        Ok(ManageSaleResult::Err(e)) => {
            let swap_id = swap.get_swap_id();
            debug!(
                "STALE SWAP JOB : Swap ID :: {swap_id:?} - Failed to withdraw forward swap deposit: {e:?}"
            );
            swap.update_status(
                SwapStatus::Forward(
                    SwapStatusForward::DepositRecoveryFailed(
                        Box::new(original_error.clone()),
                        DepositRecoveryError::CantRecover(format!("{e:?}"))
                    )
                )
            );
            return Err(());
        }
        Err(e) => {
            let swap_id = swap.get_swap_id();
            debug!(
                "STALE SWAP JOB : Swap ID :: {swap_id:?} - Failed to withdraw forward swap deposit: {e:?}"
            );
            swap.update_status(
                SwapStatus::Forward(
                    SwapStatusForward::DepositRecoveryFailed(
                        Box::new(original_error.clone()),
                        DepositRecoveryError::CallError(format!("{e:?}"))
                    )
                )
            );
            return Err(());
        }
    }
}

fn valid_for_mint(current_swap_details: &SwapDetailForward) -> Result<(), ()> {
    let current_swap_status = &current_swap_details.status;
    if current_swap_status != &SwapStatusForward::MintRequest {
        Err(())
    } else {
        Ok(())
    }
}

fn valid_for_bid(current_swap_details: &SwapDetailForward) -> Result<(), ()> {
    let current_swap_status = &current_swap_details.status;
    if current_swap_status != &SwapStatusForward::BidRequest {
        Err(())
    } else {
        Ok(())
    }
}
fn valid_init_state(current_swap_details: &SwapDetailForward) -> Result<(), ()> {
    let current_swap_status = &current_swap_details.status;
    if current_swap_status != &SwapStatusForward::Init {
        Err(())
    } else {
        Ok(())
    }
}
fn valid_for_burn_fees(current_swap_details: &SwapDetailForward) -> Result<(), ()> {
    let current_swap_status = &current_swap_details.status;
    if current_swap_status != &SwapStatusForward::BurnFeesRequest {
        Err(())
    } else {
        Ok(())
    }
}
