use crate::client::icrc1::client::balance_of;
use crate::client::icrc1::icrc1_total_supply;
use crate::client::origyn_nft_reference::client::{
    get_token_id_as_nat, icrc7_owner_of, market_transfer_nft_origyn,
};
use crate::gldt_swap_suite::nft_utils;
use crate::gldt_swap_suite::{init, CanisterIds, PrincipalIds, TestEnv};
use crate::utils::tick_n_blocks;

use candid::{Nat, Principal};
use gldt_swap_common::gldt::{GldtTokenSpec, GLDT_TX_FEE};
use gldt_swap_common::swap::{SwapInfo, SwapStatusForward};
use icrc_ledger_types::icrc1::account::Account;
use origyn_nft_reference::origyn_nft_reference_canister::{
    Account as OrigynAccount, AskFeature, MarketTransferRequest, PricingConfigShared, SalesConfig,
};
use pocket_ic::PocketIc;

use crate::client::gldt_swap::swap_nft_for_tokens;
use std::{array::TryFromSliceError, time::Duration};

use canister_time::{timestamp_millis, MINUTE_IN_MS};
use gldt_swap_common::{
    gldt::GldtNumTokens,
    nft::NftID,
    swap::{BidFailError, SwapDetailForward, SwapErrorForward, SwapId, SwapIndex},
};

use gldt_swap_api_canister::remove_intent_to_swap::RemoveIntentToSwapError;
use origyn_nft_reference::origyn_nft_reference_canister::{
    AuctionStateSharedStatus, EscrowReceipt, MarketTransferRequestReponseTxnType,
    MarketTransferResult, SaleInfoRequest, SaleInfoResponse, SaleInfoResult,
};

use crate::client::{
    gldt_swap::{get_swap, insert_fake_swap, remove_intent_to_swap},
    icrc1::client::transfer,
    origyn_nft_reference::sale_info_nft_origyn,
};

fn init_nft_with_premint_nft(
    pic: &mut PocketIc,
    origyn_nft: Principal,
    originator: Principal,
    net_principal: Principal,
    nft_owner: Principal,
    nft_name: String,
) -> bool {
    nft_utils::build_standard_nft(
        pic,
        nft_name.clone(),
        origyn_nft.clone(),
        origyn_nft.clone(),
        originator.clone(),
        Nat::from(1024 as u32),
        false,
        net_principal.clone(),
    );

    let mint_return: origyn_nft_reference::origyn_nft_reference_canister::OrigynTextResult =
        crate::client::origyn_nft_reference::client::mint_nft_origyn(
            pic,
            origyn_nft.clone(),
            Some(net_principal.clone()),
            (
                nft_name.clone(),
                OrigynAccount::Account {
                    owner: nft_owner.clone(),
                    sub_account: None,
                },
            ),
        );

    println!("mint_return: {:?}", mint_return);

    match mint_return {
        origyn_nft_reference::origyn_nft_reference_canister::OrigynTextResult::Ok(_) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use gldt_swap_api_canister::swap_nft_for_tokens::{NftInvalidError, SwapNftForTokensErrors};
    use gldt_swap_common::{
        gldt::GLDT_LEDGER_FEE_ACCOUNT,
        swap::{
            BurnFeesError, ImpossibleErrorReason, MintError, NotificationError, SwapStatus,
            TransferFailReason, STALE_SWAP_TIME_THRESHOLD_MINUTES,
        },
    };

    use crate::client::{gldt_swap::recover_stuck_swap, icrc1_icrc2_token::icrc1_balance_of};

    use super::*;
    #[test]
    pub fn forward_swap_basic_only() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    originator,
                    nft_owner,
                    ..
                },
        } = env;
        tick_n_blocks(pic, 2);

        let pre_swap_gldt_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        let mut swap_id: SwapId = SwapId(NftID(Nat::from(0u64)), SwapIndex::from(0u64));
        let res = swap_nft_for_tokens(
            pic,
            nft_owner,
            gldt_swap,
            &vec![(NftID(token_id_as_nat.clone()), origyn_nft)],
        );
        match res {
            Ok(ids) => {
                swap_id = ids[0].clone();
            }
            Err(e) => {
                println!("/// intent to swap errors : {e:?}");
            }
        }

        // verify swap got inserted with init state
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Init);
        }
        // 2. start the forward swap
        let market_args = MarketTransferRequest {
            token_id: "1".to_string(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Ask(Some(vec![
                    AskFeature::Token(GldtTokenSpec::new(gldt_ledger).get_token_spec()),
                    AskFeature::BuyNow(Nat::from(10_002_000_000u64)),
                    AskFeature::Notify(vec![gldt_swap]),
                    AskFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                    AskFeature::AllowList(vec![gldt_swap]),
                ])),
                escrow_receipt: None,
            },
        };
        market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);
        tick_n_blocks(pic, 100);
        // check swap completed
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Complete);
        }

        // check the total supply increased
        let post_sale_total_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        let expected_supply = pre_swap_gldt_supply + Nat::from(10_000_000_000u64); // mint incurs fee && nft escrow has to transfer to user ( x 2 transactions )
        assert_eq!(post_sale_total_supply, expected_supply);

        // check user got their gldt tokens
        let user_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: nft_owner.clone(),
                subaccount: None,
            },
        );
        assert_eq!(user_balance, Nat::from(10_000_000_000u64));

        // ensure canister now owns nft
        let owner_of = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat.clone()],
        );
        assert_eq!(
            owner_of.get(0).unwrap().clone().unwrap().owner.to_string(),
            gldt_swap.to_string()
        );
    }

    #[test]
    pub fn forward_swap_should_fail_if_user_doesnt_own_the_nft() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    originator,
                    nft_owner,
                    controller,
                },
        } = env;
        tick_n_blocks(pic, 2);

        let pre_swap_gldt_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        let res = swap_nft_for_tokens(
            pic,
            controller, // controlelr doesn't own the nft
            gldt_swap,
            &vec![(NftID(token_id_as_nat.clone()), origyn_nft)],
        );

        matches!(
            res,
            Err(SwapNftForTokensErrors::NftValidationErrors((_, _)))
        );
    }

    #[test]
    pub fn swap_nft_for_tokens_should_fail_if_invalid_nft_canister_supplied() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    originator,
                    nft_owner,
                    ..
                },
        } = env;
        tick_n_blocks(pic, 2);

        // 1. setup nft and verify owner

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        let mut swap_id: SwapId = SwapId(NftID(Nat::from(0u64)), SwapIndex::from(0u64));
        let res = swap_nft_for_tokens(
            pic,
            nft_owner,
            gldt_swap,
            &vec![(NftID(token_id_as_nat.clone()), Principal::anonymous())], // use annoymous principal
        );
        match res {
            Ok(r) => {
                assert_eq!(true, false)
            }
            Err(e) => {
                matches!(e, SwapNftForTokensErrors::ContainsInvalidNftCanister(_));
            }
        }
    }

    #[test]
    pub fn forward_swap_fails_if_market_transfer_properties_are_invalid() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    originator,
                    nft_owner,
                    ..
                },
        } = env;
        tick_n_blocks(pic, 2);

        let pre_swap_gldt_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        let mut swap_id: SwapId = SwapId(NftID(Nat::from(0u64)), SwapIndex::from(0u64));
        let res = swap_nft_for_tokens(
            pic,
            nft_owner,
            gldt_swap,
            &vec![(NftID(token_id_as_nat.clone()), origyn_nft)],
        );
        match res {
            Ok(ids) => {
                swap_id = ids[0].clone();
            }
            Err(e) => {
                println!("/// intent to swap errors : {e:?}");
            }
        }

        // verify swap got inserted with init state
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Init);
        }
        // 2. start the forward swap
        let market_args = MarketTransferRequest {
            token_id: "1".to_string(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Ask(Some(vec![
                    AskFeature::Token(GldtTokenSpec::new(gldt_ledger).get_token_spec()),
                    AskFeature::BuyNow(Nat::from(10_002_000_000u64)),
                    AskFeature::Notify(vec![gldt_swap]),
                    AskFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                    AskFeature::AllowList(vec![gldt_swap, Principal::anonymous()]),
                ])),
                escrow_receipt: None,
            },
        };
        market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);
        tick_n_blocks(pic, 10);
        // check swap completed
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(
                details.status,
                SwapStatusForward::Failed(SwapErrorForward::NotificationFailed(
                    NotificationError::TooManyPrincipalsInAllowList
                ))
            );
        }

        // // check the total supply increased
        // let post_sale_total_supply = icrc1_total_supply(
        //     pic,
        //     Principal::anonymous(),
        //     gldt_ledger,
        //     &()
        // );
        // let expected_supply =
        //     pre_swap_gldt_supply + Nat::from(10_000_000_000u64) - Nat::from(GLDT_TX_FEE * 2); // mint incurs fee && nft escrow has to transfer to user ( x 2 transactions )
        // assert_eq!(post_sale_total_supply, expected_supply);

        // // check user got their gldt tokens
        // let user_balance = balance_of(pic, gldt_ledger, Account {
        //     owner: nft_owner.clone(),
        //     subaccount: None,
        // });
        // assert_eq!(user_balance, Nat::from(10_000_000_000u64) - Nat::from(GLDT_TX_FEE * 2));

        // // ensure canister now owns nft
        // let owner_of = icrc7_owner_of(
        //     pic,
        //     origyn_nft.clone(),
        //     net_principal.clone(),
        //     vec![token_id_as_nat.clone()]
        // );
        // assert_eq!(
        //     owner_of.get(0).unwrap().clone().unwrap().owner.to_string(),
        //     gldt_swap.to_string()
        // );
    }

    #[test]
    pub fn forward_swap_basic_multiple_nfts() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    originator,
                    nft_owner,
                    ..
                },
        } = env;

        let pre_swap_gldt_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );
        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "2".to_string(),
        );
        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "3".to_string(),
        );

        // verify first owner
        let token_id_as_nat_1 = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        let owner_of_1 = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat_1.clone()],
        );
        assert_eq!(
            owner_of_1
                .get(0)
                .unwrap()
                .clone()
                .unwrap()
                .owner
                .to_string(),
            nft_owner.to_string()
        );

        // verify second owner
        let token_id_as_nat_2 = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "2".to_string(),
        );

        let owner_of_2 = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat_2.clone()],
        );
        assert_eq!(
            owner_of_2
                .get(0)
                .unwrap()
                .clone()
                .unwrap()
                .owner
                .to_string(),
            nft_owner.to_string()
        );
        let token_id_as_nat_3 = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "3".to_string(),
        );

        let owner_of_3 = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat_3.clone()],
        );
        assert_eq!(
            owner_of_3
                .get(0)
                .unwrap()
                .clone()
                .unwrap()
                .owner
                .to_string(),
            nft_owner.to_string()
        );

        let mut swap_id_1: SwapId = SwapId(NftID(Nat::from(0u64)), SwapIndex::from(0u64));
        let mut swap_id_2: SwapId = SwapId(NftID(Nat::from(0u64)), SwapIndex::from(1u64));
        let mut swap_id_3: SwapId = SwapId(NftID(Nat::from(0u64)), SwapIndex::from(2u64));
        let res = swap_nft_for_tokens(
            pic,
            nft_owner,
            gldt_swap,
            &vec![
                (NftID(token_id_as_nat_1.clone()), origyn_nft),
                (NftID(token_id_as_nat_2.clone()), origyn_nft),
                (NftID(token_id_as_nat_3.clone()), origyn_nft),
            ],
        );
        match res {
            Ok(ids) => {
                let mut id_iter = ids.iter().enumerate();
                swap_id_1 = id_iter.next().unwrap().1.clone();
                swap_id_2 = id_iter.next().unwrap().1.clone();
                swap_id_3 = id_iter.next().unwrap().1.clone();
            }
            Err(e) => {
                println!("/// intent to swap errors : {e:?}");
            }
        }
        tick_n_blocks(pic, 2);

        // verify swap got inserted with init state
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id_1).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Init);
        }
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id_2).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Init);
        }
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id_3).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Init);
        }
        pic.advance_time(Duration::from_nanos(5000));
        tick_n_blocks(pic, 1);
        // 2. start the forward swap
        let market_args = MarketTransferRequest {
            token_id: "1".to_string(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Ask(Some(vec![
                    AskFeature::Token(GldtTokenSpec::new(gldt_ledger).get_token_spec()),
                    AskFeature::BuyNow(Nat::from(10_002_000_000u64)),
                    AskFeature::Notify(vec![gldt_swap]),
                    AskFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                    AskFeature::AllowList(vec![gldt_swap]),
                ])),
                escrow_receipt: None,
            },
        };
        pic.advance_time(Duration::from_millis(1000));
        tick_n_blocks(pic, 1);
        market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);
        let market_args = MarketTransferRequest {
            token_id: "2".to_string(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Ask(Some(vec![
                    AskFeature::Token(GldtTokenSpec::new(gldt_ledger).get_token_spec()),
                    AskFeature::BuyNow(Nat::from(10_002_000_000u64)),
                    AskFeature::Notify(vec![gldt_swap]),
                    AskFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                    AskFeature::AllowList(vec![gldt_swap]),
                ])),
                escrow_receipt: None,
            },
        };
        pic.advance_time(Duration::from_millis(10000));
        tick_n_blocks(pic, 1);
        market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);
        let market_args = MarketTransferRequest {
            token_id: "3".to_string(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Ask(Some(vec![
                    AskFeature::Token(GldtTokenSpec::new(gldt_ledger).get_token_spec()),
                    AskFeature::BuyNow(Nat::from(10_002_000_000u64)),
                    AskFeature::Notify(vec![gldt_swap]),
                    AskFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                    AskFeature::AllowList(vec![gldt_swap]),
                ])),
                escrow_receipt: None,
            },
        };
        market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);

        tick_n_blocks(pic, 199);

        // check swap completed
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id_1).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Complete);
        }
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id_2).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Complete);
        }
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id_3).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Complete);
        }

        // check the total supply increased
        let post_sale_total_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        let expected_supply = pre_swap_gldt_supply + Nat::from(10_000_000_000u64 * 3); // mint incurs fee && nft escrow has to transfer to user ( x 2 transactions )
        assert_eq!(post_sale_total_supply, expected_supply);

        // check user got their gldt tokens
        let user_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: nft_owner.clone(),
                subaccount: None,
            },
        );
        assert_eq!(user_balance, Nat::from(10_000_000_000u64 * 3));

        // ensure canister now owns nft
        let owner_of_1 = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat_1.clone()],
        );
        assert_eq!(
            owner_of_1
                .get(0)
                .unwrap()
                .clone()
                .unwrap()
                .owner
                .to_string(),
            gldt_swap.to_string()
        );
        let owner_of_2 = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat_2.clone()],
        );
        assert_eq!(
            owner_of_2
                .get(0)
                .unwrap()
                .clone()
                .unwrap()
                .owner
                .to_string(),
            gldt_swap.to_string()
        );
        let owner_of_3 = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat_3.clone()],
        );
        assert_eq!(
            owner_of_3
                .get(0)
                .unwrap()
                .clone()
                .unwrap()
                .owner
                .to_string(),
            gldt_swap.to_string()
        );
    }

    #[test]
    pub fn remove_intent_to_swap_happy_path() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    originator,
                    nft_owner,
                    ..
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        let owner_of = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat.clone()],
        );
        assert_eq!(
            owner_of.get(0).unwrap().clone().unwrap().owner.to_string(),
            nft_owner.to_string()
        );
        let mut swap_id: SwapId = SwapId(NftID(Nat::from(0u64)), SwapIndex::from(0u64));
        let res = swap_nft_for_tokens(
            pic,
            nft_owner,
            gldt_swap,
            &vec![(NftID(token_id_as_nat.clone()), origyn_nft)],
        );
        match res {
            Ok(ids) => {
                swap_id = ids[0].clone();
            }
            Err(e) => {
                println!("/// intent to swap errors : {e:?}");
            }
        }
        tick_n_blocks(pic, 2);

        // verify swap got inserted with init state
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Init);
        }

        // cancel the swap
        remove_intent_to_swap(pic, nft_owner, gldt_swap, &res.0).unwrap();

        // check swap completed
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id).is_none();
        assert_eq!(res, true);
    }

    #[test]
    pub fn remove_intent_to_swap_fails_if_nft_canister_is_processing() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    originator,
                    nft_owner,
                    ..
                },
        } = env;

        icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        let owner_of = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat.clone()],
        );
        assert_eq!(
            owner_of.get(0).unwrap().clone().unwrap().owner.to_string(),
            nft_owner.to_string()
        );
        let mut swap_id: SwapId = SwapId(NftID(Nat::from(0u64)), SwapIndex::from(0u64));
        let res = swap_nft_for_tokens(
            pic,
            nft_owner,
            gldt_swap,
            &vec![(NftID(token_id_as_nat.clone()), origyn_nft)],
        );
        match res {
            Ok(ids) => {
                swap_id = ids[0].clone();
            }
            Err(e) => {
                println!("/// intent to swap errors : {e:?}");
            }
        }
        tick_n_blocks(pic, 2);

        // verify swap got inserted with init state
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Init);
        }

        // 2. start the forward swap but notify a random principal so we can test remove_intent_to_swap fails
        let market_args = MarketTransferRequest {
            token_id: "1".to_string(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Ask(Some(vec![
                    AskFeature::Token(GldtTokenSpec::new(gldt_ledger).get_token_spec()),
                    AskFeature::BuyNow(Nat::from(10_020_000_000u64)),
                    AskFeature::Notify(vec![Principal::anonymous()]),
                    AskFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                    AskFeature::AllowList(vec![]),
                ])),
                escrow_receipt: None,
            },
        };
        market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);
        // pic.advance_time(Duration::from_secs(20));
        tick_n_blocks(pic, 20);
        // cancel the swap
        let res = remove_intent_to_swap(pic, nft_owner, gldt_swap, &res.0);
        assert_eq!(res.is_err(), true);
        match res {
            Ok(_) => {}
            Err(e) => assert_eq!(e, RemoveIntentToSwapError::InProgress),
        }
    }

    #[test]
    pub fn forward_swap_returns_lock_error_if_the_same_nft_is_already_being_swapped() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    originator,
                    nft_owner,
                    ..
                },
        } = env;
        tick_n_blocks(pic, 2);

        let pre_swap_gldt_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        let mut swap_id: SwapId = SwapId(NftID(Nat::from(0u64)), SwapIndex::from(0u64));
        let res = swap_nft_for_tokens(
            pic,
            nft_owner,
            gldt_swap,
            &vec![(NftID(token_id_as_nat.clone()), origyn_nft)],
        );
        match res {
            Ok(ids) => {
                swap_id = ids[0].clone();
            }
            Err(e) => {
                println!("/// intent to swap errors : {e:?}");
            }
        }

        let res = swap_nft_for_tokens(
            pic,
            nft_owner,
            gldt_swap,
            &vec![(NftID(token_id_as_nat.clone()), origyn_nft)],
        );
        match res {
            Ok(ids) => {
                swap_id = ids[0].clone();
            }
            Err(e) => {
                match e {
                    gldt_swap_api_canister::swap_nft_for_tokens::SwapNftForTokensErrors::NftValidationErrors(
                        validation_error,
                    ) => {
                        println!("{validation_error:?}");

                        assert_eq!(validation_error.1[0].0.0, token_id_as_nat);
                        assert_eq!(validation_error.1[0].1[0], NftInvalidError::AlreadyLocked);
                    }
                    _ => {
                        println!("/// fail expected already locked error but got : {e:?}");
                        assert_eq!(true, false);
                    }
                }
            }
        }
    }

    #[test]
    fn forward_swaps_that_get_stuck_at_bid_request_are_handled_correctly() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                    ..
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );
        // let nft_id = NftID(token_id_as_nat.clone());

        // 2. create an active sale on the nft canister - but dont notify the swap canister
        let market_args = MarketTransferRequest {
            token_id: "1".to_string(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Ask(Some(vec![
                    AskFeature::Token(GldtTokenSpec::new(gldt_ledger).get_token_spec()),
                    AskFeature::BuyNow(Nat::from(10_002_000_000u64)),
                    AskFeature::Notify(vec![Principal::anonymous()]),
                    AskFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                    AskFeature::AllowList(vec![gldt_swap]),
                ])),
                escrow_receipt: None,
            },
        };

        let res = market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);

        // 3. get the sale_id
        let sale_id = match res {
            MarketTransferResult::Ok(res_ok) => match res_ok.txn_type {
                MarketTransferRequestReponseTxnType::SaleOpened { sale_id, .. } => sale_id,
                _ => "bad_sale".to_string(),
            },
            MarketTransferResult::Err(e) => {
                println!("//// there was an error {e:?}");
                "bad_sale".to_string()
            }
        };
        // 4. get the escrow account
        let args = &SaleInfoRequest::EscrowInfo(EscrowReceipt {
            token: GldtTokenSpec::new(gldt_ledger).get_token_spec(),
            token_id: "1".to_string(),
            seller: OrigynAccount::Account {
                owner: nft_owner,
                sub_account: None,
            },
            buyer: OrigynAccount::Account {
                owner: gldt_swap,
                sub_account: None,
            },
            amount: Nat::from(10_020_000_000u64),
        });
        let res = sale_info_nft_origyn(pic, Principal::anonymous(), origyn_nft, &args);
        let escrow_sub_account = match res {
            SaleInfoResult::Ok(res_ok) => match res_ok {
                SaleInfoResponse::EscrowInfo(escrow_info) => {
                    let b: Result<[u8; 32], TryFromSliceError> =
                        escrow_info.account.sub_account.as_slice().try_into();
                    match b {
                        Ok(sub_account) => sub_account,
                        Err(_) => panic!("failed to parse sub account"),
                    }
                }
                _ => {
                    panic!("escrow account not found")
                }
            },
            SaleInfoResult::Err(_) => {
                panic!("escrow account not found")
            }
        };
        // 5. get the total gldt supply before the mint occurs
        let pre_swap_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        // 6. mint gldt to the escrow account of the sale
        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: origyn_nft,
                subaccount: Some(escrow_sub_account.clone()),
            },
            10_002_000_000u128, // we intentionally minus 2 transaction fees because
        )
        .unwrap();

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::BidRequest,
                sale_id: sale_id.clone(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: escrow_sub_account,
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 1);

        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), SwapIndex::from(0u64)),
        )
        .unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::BidRequest);
        }

        // 8. advance time to trigger the cron job remove_stale_swaps.rs
        // for some reason i must trigger the cron job this way
        // the cron should detect that the swap is expired by the following criteria
        // - time ( more than 3 minutes has passed )
        // - & the sale has expired
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        tick_n_blocks(pic, 1);
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 1));
        tick_n_blocks(pic, 2);

        // 9. verify the sale is expired
        let sale_info_status = match sale_info_nft_origyn(
            pic,
            nft_owner.clone(),
            origyn_nft.clone(),
            &SaleInfoRequest::Status(sale_id.clone()),
        ) {
            SaleInfoResult::Ok(res_ok) => {
                match res_ok {
                    SaleInfoResponse::Status(status) => {
                        match status {
                            Some(s) => {
                                match s.sale_type {
                                    origyn_nft_reference::origyn_nft_reference_canister::SaleStatusSharedSaleType::Auction(
                                        auction,
                                    ) => {
                                        auction.status
                                    }
                                    // _ => AuctionStateSharedStatus::NotStarted,
                                }
                            }
                            None => AuctionStateSharedStatus::NotStarted,
                        }
                    }
                    _ => AuctionStateSharedStatus::NotStarted,
                }
            }
            SaleInfoResult::Err(_) => AuctionStateSharedStatus::NotStarted,
        };
        assert_eq!(sale_info_status, AuctionStateSharedStatus::Closed);

        // 10. verify the cron forced the swap to a failed expired state
        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), SwapIndex::from(0u64)),
        )
        .unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(
                details.status,
                SwapStatusForward::Failed(SwapErrorForward::Expired(Box::new(
                    SwapStatusForward::BidRequest
                )))
            );
        }
        // 11. verify the nft canister sent back the tokens to the swap canister ( burned them ) and so the supply of gldt is the same as when we started
        let post_fail_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(pre_swap_supply, post_fail_supply);

        // 12. verify the owner of the nft is still the user
        let owner_of = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat.clone()],
        );
        assert_eq!(
            owner_of.get(0).unwrap().clone().unwrap().owner.to_string(),
            nft_owner.to_string()
        );
    }

    #[test]
    fn forward_swaps_that_get_stuck_at_bid_failed_are_handled_correctly() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );
        // let nft_id = NftID(token_id_as_nat.clone());

        // 2. create an active sale on the nft canister - but dont notify the swap canister
        let market_args = MarketTransferRequest {
            token_id: "1".to_string(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Ask(Some(vec![
                    AskFeature::Token(GldtTokenSpec::new(gldt_ledger).get_token_spec()),
                    AskFeature::BuyNow(Nat::from(10_002_000_000u64)),
                    AskFeature::Notify(vec![Principal::anonymous()]),
                    AskFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                    AskFeature::AllowList(vec![gldt_swap]),
                ])),
                escrow_receipt: None,
            },
        };

        let res = market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);

        // 3. get the sale_id
        let sale_id = match res {
            MarketTransferResult::Ok(res_ok) => match res_ok.txn_type {
                MarketTransferRequestReponseTxnType::SaleOpened { sale_id, .. } => sale_id,
                _ => "bad_sale".to_string(),
            },
            MarketTransferResult::Err(e) => {
                println!("//// there was an error {e:?}");
                "bad_sale".to_string()
            }
        };
        // 4. get the escrow account
        let args = &SaleInfoRequest::EscrowInfo(EscrowReceipt {
            token: GldtTokenSpec::new(gldt_ledger).get_token_spec(),
            token_id: "1".to_string(),
            seller: OrigynAccount::Account {
                owner: nft_owner,
                sub_account: None,
            },
            buyer: OrigynAccount::Account {
                owner: gldt_swap,
                sub_account: None,
            },
            amount: Nat::from(10_002_000_000u64),
        });
        let res = sale_info_nft_origyn(pic, Principal::anonymous(), origyn_nft, &args);
        let escrow_sub_account = match res {
            SaleInfoResult::Ok(res_ok) => match res_ok {
                SaleInfoResponse::EscrowInfo(escrow_info) => {
                    let b: Result<[u8; 32], TryFromSliceError> =
                        escrow_info.account.sub_account.as_slice().try_into();
                    match b {
                        Ok(sub_account) => sub_account,
                        Err(_) => panic!("failed to parse sub account"),
                    }
                }
                _ => {
                    panic!("escrow account not found")
                }
            },
            SaleInfoResult::Err(_) => {
                panic!("escrow account not found")
            }
        };
        // 5. get the total gldt supply before the mint occurs
        let pre_swap_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        // 6. mint gldt to the escrow account of the sale
        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: origyn_nft,
                subaccount: Some(escrow_sub_account.clone()),
            },
            10_002_000_000u128, // we intentionally minus 2 transaction fees because
        )
        .unwrap();

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::BidFail(BidFailError::TransferFailed(
                    "something went wrong".to_string(),
                )),
                sale_id: sale_id.clone(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: escrow_sub_account,
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 5);

        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), SwapIndex::from(0u64)),
        )
        .unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(
                details.status,
                SwapStatusForward::BidFail(BidFailError::TransferFailed(
                    "something went wrong".to_string()
                ))
            );
        }

        // 8. advance time to trigger the cron job remove_stale_swaps.rs
        // for some reason i must trigger the cron job this way
        // the cron should detect that the swap is expired by the following criteria
        // - time ( more than 3 minutes has passed )
        // - & the sale has expired
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        tick_n_blocks(pic, 3);
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 1));
        tick_n_blocks(pic, 3);

        // 9. verify the sale is expired
        let sale_info_status = match sale_info_nft_origyn(
            pic,
            nft_owner.clone(),
            origyn_nft.clone(),
            &SaleInfoRequest::Status(sale_id.clone()),
        ) {
            SaleInfoResult::Ok(res_ok) => {
                match res_ok {
                    SaleInfoResponse::Status(status) => {
                        match status {
                            Some(s) => {
                                match s.sale_type {
                                    origyn_nft_reference::origyn_nft_reference_canister::SaleStatusSharedSaleType::Auction(
                                        auction,
                                    ) => {
                                        auction.status
                                    }
                                    // _ => AuctionStateSharedStatus::NotStarted,
                                }
                            }
                            None => AuctionStateSharedStatus::NotStarted,
                        }
                    }
                    _ => AuctionStateSharedStatus::NotStarted,
                }
            }
            SaleInfoResult::Err(_) => AuctionStateSharedStatus::NotStarted,
        };
        assert_eq!(sale_info_status, AuctionStateSharedStatus::Closed);

        // 10. verify the cron forced the swap to a failed expired state
        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), SwapIndex::from(0u64)),
        )
        .unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(
                details.status,
                SwapStatusForward::Failed(SwapErrorForward::BidFailed(
                    BidFailError::TransferFailed("something went wrong".to_string())
                ))
            );
        }
        // 11. verify the nft canister sent back the tokens to the swap canister ( burned them ) and so the supply of gldt is the same as when we started
        let post_fail_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(pre_swap_supply, post_fail_supply);

        // 12. verify the owner of the nft is still the user
        let owner_of = icrc7_owner_of(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            vec![token_id_as_nat.clone()],
        );
        assert_eq!(
            owner_of.get(0).unwrap().clone().unwrap().owner.to_string(),
            nft_owner.to_string()
        );
    }

    #[test]
    fn forward_swap_stuck_at_burn_fees_status_is_auto_recovered() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        // 6. mint 2x gldt to the gldt ledger fee account
        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
            },
            (GLDT_TX_FEE * 2) as u128,
        )
        .unwrap();

        let pre_swap_total_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::BurnFeesFailed(BurnFeesError::TransferFailed(
                    TransferFailReason::CallError("simulate_call_error".to_string()),
                )),
                sale_id: "somerandomsale".to_string(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), SwapIndex::from(0u64)),
        )
        .unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(
                details.status,
                SwapStatusForward::BurnFeesFailed(BurnFeesError::TransferFailed(
                    TransferFailReason::CallError("simulate_call_error".to_string())
                ))
            );
        }
        // wait for swap to be considered stuck by the backend
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);

        // 7. swap should be completed
        let user_swap = get_swap(pic, Principal::anonymous(), gldt_swap, &res.0);
        assert_eq!(&user_swap.is_some(), &true);
        if let SwapInfo::Forward(details) = user_swap.unwrap().1 {
            assert_eq!(details.status, SwapStatusForward::Complete);
        }

        let post_swap_total_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        assert_eq!(
            post_swap_total_supply,
            pre_swap_total_supply - Nat::from(2 * GLDT_TX_FEE)
        );

        // check the burn fees got burnt
    }

    #[test]
    fn forward_swap_stuck_at_init_gets_expired() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::Init,
                sale_id: "somerandomsale".to_string(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        // wait for swap to be considered stuck by the backend
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);

        // 7. swap should be completed
        let user_swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), Nat::from(0u64)),
        );
        if let SwapInfo::Forward(details) = user_swap.unwrap().1 {
            assert_eq!(
                details.status,
                SwapStatusForward::Failed(SwapErrorForward::Expired(Box::new(
                    SwapStatusForward::Init
                )))
            );
        }
        // check the burn fees got burnt
    }

    #[test]
    fn forward_swap_stuck_at_notification_in_progress() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::NotificationInProgress,
                sale_id: "somerandomsale".to_string(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        // wait for swap to be considered stuck by the backend
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);

        // 7. swap should be completed
        let user_swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), Nat::from(0u64)),
        );
        if let SwapInfo::Forward(details) = user_swap.unwrap().1 {
            assert_eq!(details.status, SwapStatusForward::NotificationInProgress);
        }
        // check the burn fees got burnt
    }
    #[test]
    fn forward_swap_stuck_at_notification_failed() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::NotificationFailed(
                    NotificationError::InvalidTokenAmount,
                ),
                sale_id: "somerandomsale".to_string(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        // wait for swap to be considered stuck by the backend
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);

        // 7. swap should be completed
        let user_swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), Nat::from(0u64)),
        );
        if let SwapInfo::Forward(details) = user_swap.unwrap().1 {
            assert_eq!(
                details.status,
                SwapStatusForward::Failed(SwapErrorForward::Expired(Box::new(
                    SwapStatusForward::NotificationFailed(NotificationError::InvalidTokenAmount)
                )))
            );
        }
        // check the burn fees got burnt
    }
    #[test]
    fn forward_swap_stuck_at_mint_request() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::MintRequest,
                sale_id: "somerandomsale".to_string(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        // wait for swap to be considered stuck by the backend
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);

        // 7. swap should be completed
        let user_swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), Nat::from(0u64)),
        );
        if let SwapInfo::Forward(details) = user_swap.unwrap().1 {
            assert_eq!(
                details.status,
                SwapStatusForward::Failed(SwapErrorForward::Expired(Box::new(
                    SwapStatusForward::MintRequest
                )))
            );
        }
        // check the burn fees got burnt
    }
    #[test]
    fn forward_swap_stuck_at_mint_in_progress() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::MintInProgress,
                sale_id: "somerandomsale".to_string(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        // wait for swap to be considered stuck by the backend
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);

        // 7. swap should be completed
        let user_swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), Nat::from(0u64)),
        );
        if let SwapInfo::Forward(details) = user_swap.unwrap().1 {
            assert_eq!(details.status, SwapStatusForward::MintInProgress);
        }
        // check the burn fees got burnt
    }
    #[test]
    fn forward_swap_stuck_at_mint_failed() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::MintFailed(MintError::UnexpectedError(
                    ImpossibleErrorReason::AmountNotFound,
                )),
                sale_id: "somerandomsale".to_string(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        // wait for swap to be considered stuck by the backend
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);

        // 7. swap should be completed
        let user_swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), Nat::from(0u64)),
        );
        if let SwapInfo::Forward(details) = user_swap.unwrap().1 {
            assert_eq!(
                details.status,
                SwapStatusForward::Failed(SwapErrorForward::Expired(Box::new(
                    SwapStatusForward::MintFailed(MintError::UnexpectedError(
                        ImpossibleErrorReason::AmountNotFound
                    ))
                )))
            );
        }
        // check the burn fees got burnt
    }
    #[test]
    fn forward_swap_stuck_at_bid_in_progress() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::BidInProgress,
                sale_id: "somerandomsale".to_string(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        // wait for swap to be considered stuck by the backend
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);

        // 7. swap should be completed
        let user_swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), Nat::from(0u64)),
        );
        if let SwapInfo::Forward(details) = user_swap.unwrap().1 {
            assert_eq!(details.status, SwapStatusForward::BidInProgress);
        }
        // check the burn fees got burnt
    }

    #[test]
    fn forward_swap_stuck_at_burn_fees_request() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::BurnFeesRequest,
                sale_id: "somerandomsale".to_string(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        // transfer some gldt to a random principal to simulate the 2 x gldt fee that ends up in the fee collector account.
        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT.clone()),
            },
            3_000_000u128, // we intentionally minus 2 transaction fees because
        )
        .unwrap();
        tick_n_blocks(pic, 3);

        // wait for swap to be considered stuck by the backend
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);

        // 7. swap should be completed
        let user_swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), Nat::from(0u64)),
        );
        if let SwapInfo::Forward(details) = user_swap.unwrap().1 {
            assert_eq!(details.status, SwapStatusForward::Complete);
        }
        // check the burn fees got burnt
    }

    #[test]
    fn forward_swap_stuck_at_burn_fees_in_progress() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::BurnFeesInProgress,
                sale_id: "somerandomsale".to_string(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        // wait for swap to be considered stuck by the backend
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);

        // 7. swap should be completed
        let user_swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), Nat::from(0u64)),
        );
        if let SwapInfo::Forward(details) = user_swap.unwrap().1 {
            assert_eq!(details.status, SwapStatusForward::BurnFeesInProgress);
        }
        // check the burn fees got burnt
    }
    #[test]
    fn forward_swap_manual_recovery_of_escrow_deposit() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids:
                PrincipalIds {
                    net_principal,
                    controller,
                    originator,
                    nft_owner,
                },
        } = env;

        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );
        // let nft_id = NftID(token_id_as_nat.clone());

        // 2. create an active sale on the nft canister - but dont notify the swap canister
        let market_args = MarketTransferRequest {
            token_id: "1".to_string(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Ask(Some(vec![
                    AskFeature::Token(GldtTokenSpec::new(gldt_ledger).get_token_spec()),
                    AskFeature::BuyNow(Nat::from(10_002_000_000u64)),
                    AskFeature::Notify(vec![Principal::anonymous()]),
                    AskFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                    AskFeature::AllowList(vec![gldt_swap]),
                ])),
                escrow_receipt: None,
            },
        };

        let res = market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);

        // 3. get the sale_id
        let sale_id = match res {
            MarketTransferResult::Ok(res_ok) => match res_ok.txn_type {
                MarketTransferRequestReponseTxnType::SaleOpened { sale_id, .. } => sale_id,
                _ => "bad_sale".to_string(),
            },
            MarketTransferResult::Err(e) => {
                println!("//// there was an error {e:?}");
                "bad_sale".to_string()
            }
        };
        // 4. get the escrow account
        let args = &SaleInfoRequest::EscrowInfo(EscrowReceipt {
            token: GldtTokenSpec::new(gldt_ledger).get_token_spec(),
            token_id: "1".to_string(),
            seller: OrigynAccount::Account {
                owner: nft_owner,
                sub_account: None,
            },
            buyer: OrigynAccount::Account {
                owner: gldt_swap,
                sub_account: None,
            },
            amount: Nat::from(10_002_000_000u64),
        });
        let res = sale_info_nft_origyn(pic, Principal::anonymous(), origyn_nft, &args);
        let escrow_sub_account = match res {
            SaleInfoResult::Ok(res_ok) => match res_ok {
                SaleInfoResponse::EscrowInfo(escrow_info) => {
                    let b: Result<[u8; 32], TryFromSliceError> =
                        escrow_info.account.sub_account.as_slice().try_into();
                    match b {
                        Ok(sub_account) => sub_account,
                        Err(_) => panic!("failed to parse sub account"),
                    }
                }
                _ => {
                    panic!("escrow account not found")
                }
            },
            SaleInfoResult::Err(_) => {
                panic!("escrow account not found")
            }
        };
        // 5. get the total gldt supply before the mint occurs
        let pre_swap_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        // 6. mint gldt to the escrow account of the sale
        // transfer(
        //     pic,
        //     gldt_swap,
        //     gldt_ledger,
        //     None,
        //     Account {
        //         owner: origyn_nft,
        //         subaccount: Some(escrow_sub_account.clone()),
        //     },
        //     10_002_000_000u128 // we intentionally minus 2 transaction fees because
        // ).unwrap();

        // 7. insert a fake swap that simulates a forward swap that got stuck at bid request ( just after the escrow has been completed )
        let time = timestamp_millis();
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(token_id_as_nat.clone()),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::BidFail(BidFailError::TransferFailed(
                    "something went wrong".to_string(),
                )),
                sale_id: sale_id.clone(),
                created_at: time,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: escrow_sub_account,
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 5);

        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), SwapIndex::from(0u64)),
        )
        .unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(
                details.status,
                SwapStatusForward::BidFail(BidFailError::TransferFailed(
                    "something went wrong".to_string()
                ))
            );
        }

        // 8. advance time to trigger the cron job remove_stale_swaps.rs
        // for some reason i must trigger the cron job this way
        // the cron should detect that the swap is expired by the following criteria
        // - time ( more than 3 minutes has passed )
        // - & the sale has expired
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        tick_n_blocks(pic, 3);
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 1));
        tick_n_blocks(pic, 3);

        // 9. verify the sale is expired
        let sale_info_status = match sale_info_nft_origyn(
            pic,
            nft_owner.clone(),
            origyn_nft.clone(),
            &SaleInfoRequest::Status(sale_id.clone()),
        ) {
            SaleInfoResult::Ok(res_ok) => {
                match res_ok {
                    SaleInfoResponse::Status(status) => {
                        match status {
                            Some(s) => {
                                match s.sale_type {
                                    origyn_nft_reference::origyn_nft_reference_canister::SaleStatusSharedSaleType::Auction(
                                        auction,
                                    ) => {
                                        auction.status
                                    }
                                    // _ => AuctionStateSharedStatus::NotStarted,
                                }
                            }
                            None => AuctionStateSharedStatus::NotStarted,
                        }
                    }
                    _ => AuctionStateSharedStatus::NotStarted,
                }
            }
            SaleInfoResult::Err(_) => AuctionStateSharedStatus::NotStarted,
        };
        assert_eq!(sale_info_status, AuctionStateSharedStatus::Closed);

        // 10. verify the cron forced the swap to a failed expired state
        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), SwapIndex::from(0u64)),
        )
        .unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            if !matches!(
                details.status,
                SwapStatusForward::DepositRecoveryFailed(_, _)
            ) {
                panic!("{:?} did not match", details.status);
            }
        }

        // transfer the gldt to the - note this type of error wouldn't happen but it's hard to simulate a call failure with pocketIc.
        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: origyn_nft,
                subaccount: Some(escrow_sub_account.clone()),
            },
            10_002_000_000u128, // we intentionally minus 2 transaction fees because
        )
        .unwrap();

        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        tick_n_blocks(pic, 3);
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 1));
        tick_n_blocks(pic, 3);

        recover_stuck_swap(
            pic,
            controller,
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), SwapIndex::from(0u64)),
        )
        .unwrap();
        tick_n_blocks(pic, 2);

        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(token_id_as_nat.clone()), SwapIndex::from(0u64)),
        )
        .unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(
                details.status,
                SwapStatusForward::Failed(SwapErrorForward::BidFailed(
                    BidFailError::TransferFailed("something went wrong".to_string())
                ))
            );
        }

        let post_fail_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(pre_swap_supply, post_fail_supply);
    }
}
