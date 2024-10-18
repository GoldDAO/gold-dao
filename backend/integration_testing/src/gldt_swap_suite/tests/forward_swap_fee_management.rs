use crate::client::icrc1::client::balance_of;
use crate::client::origyn_nft_reference::client::{
    get_token_id_as_nat,
    icrc7_owner_of,
    market_transfer_nft_origyn,
};
use crate::gldt_swap_suite::{ init, CanisterIds, PrincipalIds, TestEnv };
use crate::utils::tick_n_blocks;
use crate::gldt_swap_suite::nft_utils;
use crate::client::icrc1::icrc1_total_supply;

use gldt_swap_common::gldt::{ GLDT_TX_FEE, GldtTokenSpec };
use gldt_swap_common::swap::{ SwapInfo, SwapStatusForward };
use icrc_ledger_types::icrc1::account::Account;
use origyn_nft_reference::origyn_nft_reference_canister::{
    Account as OrigynAccount,
    AskFeature,
    MarketTransferRequest,
    PricingConfigShared,
    SalesConfig,
};
use candid::{ Nat, Principal };
use pocket_ic::PocketIc;

use crate::client::gldt_swap::swap_nft_for_tokens;
use std::{ array::TryFromSliceError, time::Duration };

use canister_time::{ timestamp_millis, MINUTE_IN_MS };
use gldt_swap_common::{
    gldt::GldtNumTokens,
    nft::NftID,
    swap::{ BidFailError, SwapDetailForward, SwapErrorForward, SwapId, SwapIndex },
};

use gldt_swap_api_canister::remove_intent_to_swap::RemoveIntentToSwapError;
use origyn_nft_reference::origyn_nft_reference_canister::{
    AuctionStateSharedStatus,
    EscrowReceipt,
    MarketTransferRequestReponseTxnType,
    MarketTransferResult,
    SaleInfoRequest,
    SaleInfoResponse,
    SaleInfoResult,
};

use crate::client::{
    gldt_swap::{ get_swap, insert_fake_swap, remove_intent_to_swap },
    icrc1::client::transfer,
    origyn_nft_reference::sale_info_nft_origyn,
};

fn init_nft_with_premint_nft(
    pic: &mut PocketIc,
    origyn_nft: Principal,
    originator: Principal,
    net_principal: Principal,
    nft_owner: Principal,
    nft_name: String
) -> bool {
    nft_utils::build_standard_nft(
        pic,
        nft_name.clone(),
        origyn_nft.clone(),
        origyn_nft.clone(),
        originator.clone(),
        Nat::from(1024 as u32),
        false,
        net_principal.clone()
    );

    let mint_return: origyn_nft_reference::origyn_nft_reference_canister::OrigynTextResult = crate::client::origyn_nft_reference::client::mint_nft_origyn(
        pic,
        origyn_nft.clone(),
        Some(net_principal.clone()),
        (nft_name.clone(), OrigynAccount::Account { owner: nft_owner.clone(), sub_account: None })
    );

    println!("mint_return: {:?}", mint_return);

    match mint_return {
        origyn_nft_reference::origyn_nft_reference_canister::OrigynTextResult::Ok(_) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use gldt_swap_common::swap::{ NotificationError, STALE_SWAP_TIME_THRESHOLD_MINUTES };
    use utils::consts::E8S_FEE_OGY;

    use crate::{
        client::{
            icrc1_icrc2_token::{ icrc1_transfer, icrc2_transfer_from },
            origyn_nft_reference::nft_origyn,
        },
        wasms::ORIGYN_NFT,
    };

    use super::*;
    #[test]
    pub fn forward_swap_fee_account_is_automatically_topped_up() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids: CanisterIds { origyn_nft, gldt_ledger, gldt_swap, ogy_ledger, .. },
            principal_ids: PrincipalIds { net_principal, originator, nft_owner, .. },
        } = env;
        tick_n_blocks(pic, 2);

        let pre_swap_gldt_supply = icrc1_total_supply(
            pic,
            Principal::anonymous(),
            gldt_ledger,
            &()
        );

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string()
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string()
        );

        let info_req = sale_info_nft_origyn(
            pic,
            gldt_swap,
            origyn_nft,
            &SaleInfoRequest::FeeDepositInfo(
                Some(OrigynAccount::Account {
                    owner: gldt_swap,
                    sub_account: None,
                })
            )
        );

        let account = match info_req {
            SaleInfoResult::Ok(ok_res) => {
                match ok_res {
                    SaleInfoResponse::FeeDepositInfo(fee_deposit_info) => {
                        let account = Account {
                            owner: fee_deposit_info.account.principal,
                            subaccount: Some(
                                fee_deposit_info.account.sub_account.as_slice().try_into().unwrap()
                            ),
                        };
                        account
                    }
                    _ => { panic!("Can't find account") }
                }
            }
            SaleInfoResult::Err(error) => { panic!("Can't find account {error:?}") }
        };

        let starting_ogy_balance = balance_of(pic, ogy_ledger, account);
        println!("{starting_ogy_balance:?}");
        assert_eq!(starting_ogy_balance, Nat::from(1_000_000_000_000u64)); // starting fee balance

        let transfer_amount = Nat::from(1_000_000_000u64 * 20u64);
        // reduce the balance
        let dummyAccount = Account {
            owner: origyn_nft,
            subaccount: Some([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ]),
        };

        let l = icrc1_transfer(
            pic,
            origyn_nft,
            ogy_ledger,
            &(icrc1_transfer::Args {
                from_subaccount: account.subaccount,
                to: dummyAccount,
                fee: None,
                created_at_time: None,
                memo: None,
                amount: transfer_amount.clone(),
            })
        );
        match l {
            icrc1_transfer::Response::Ok(a) => {
                println!("{a:?}");
            }
            icrc1_transfer::Response::Err(b) => {
                println!("{b:?}");
            }
        }

        tick_n_blocks(pic, 2);

        let res = balance_of(pic, ogy_ledger, account);
        assert_eq!(res, Nat::from(starting_ogy_balance.clone() - (transfer_amount + E8S_FEE_OGY)));

        // wait for cron to kick in
        // assert we have more than the threshold
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 1);
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 1);
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 1);
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 10);
        let current_balance = balance_of(pic, ogy_ledger, account);
        println!("current_balance {current_balance:?}");
        println!("starting ogy {starting_ogy_balance:?}");
        assert!(current_balance > starting_ogy_balance);
    }
}
