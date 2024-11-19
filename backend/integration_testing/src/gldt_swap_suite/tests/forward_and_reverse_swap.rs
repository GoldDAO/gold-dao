use crate::client::origyn_nft_reference::client::{
    get_token_id_as_nat, market_transfer_nft_origyn,
};
use crate::gldt_swap_suite::nft_utils;
use crate::gldt_swap_suite::{init, CanisterIds, PrincipalIds, TestEnv};
use crate::utils::tick_n_blocks;

use candid::{Nat, Principal};
use gldt_swap_api_canister::swap_tokens_for_nft::Args as SwapTokensForNftArgs;
use gldt_swap_common::gldt::GldtTokenSpec;
use gldt_swap_common::swap::{SwapInfo, SwapStatusForward};
use icrc_ledger_types::icrc1::account::Account;
use origyn_nft_reference::origyn_nft_reference_canister::{
    Account as OrigynAccount, AskFeature, MarketTransferRequest, PricingConfigShared, SalesConfig,
};
use pocket_ic::PocketIc;

use crate::client::gldt_swap::swap_nft_for_tokens;
use std::time::Duration;

use gldt_swap_common::{
    nft::NftID,
    swap::{SwapId, SwapIndex},
};

use crate::client::gldt_swap::get_swap;

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
    use canister_time::{timestamp_nanos, HOUR_IN_MS, SECOND_IN_MS};
    use gldt_swap_common::{
        gldt::{GLDT_LEDGER_FEE_ACCOUNT, GLDT_SWAP_FEE_ACCOUNT},
        swap::{SwapStatusReverse, MANAGE_GLDT_SUPPLY_INTERVAL},
    };

    use crate::client::{
        gldt_swap::{get_owned_nfts, swap_tokens_for_nft},
        icrc1::{client::balance_of, icrc1_total_supply},
        icrc1_icrc2_token::icrc2_approve,
    };

    use super::*;

    // ------
    // Why make a test for forward -> reverse -> forward?
    // ------

    // the nft canister may change the account type from OrigynAccount::Principal to OrigynAccount::Account depending on the method of transfer used.
    // this tests asserts that our logic covers this possibility by using the same NFT three times in a forward -> reverse -> forward swap to ensure the
    // account on the nft can be swapped back and forth regardless of what the nft canister does with the account type. There are less checks happening in this test
    // since we care more about the ability to transfer rather than the supply of gldt for example. Other tests cover the supply etc.

    #[test]
    pub fn forward_reverse_forward_swap() {
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
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "1".to_string(),
        );

        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            "2".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );
        let nft_id = NftID(token_id_as_nat.clone());

        let token_id_as_nat_2 = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "2".to_string(),
        );

        /////////////////////////////////
        /////  Forward swap
        /////////////////////////////////

        // nft 1
        let gldt_supply_pre = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        println!("///// presale supply : {gldt_supply_pre}");
        assert_eq!(gldt_supply_pre, Nat::from(0u64));
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

        // nft 2
        let mut swap_id: SwapId = SwapId(NftID(Nat::from(1u64)), SwapIndex::from(1u64));
        let res = swap_nft_for_tokens(
            pic,
            nft_owner,
            gldt_swap,
            &vec![(NftID(token_id_as_nat_2.clone()), origyn_nft)],
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
        market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);
        tick_n_blocks(pic, 100);
        // check swap completed
        let res = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id).unwrap();
        if let SwapInfo::Forward(details) = res.1 {
            assert_eq!(details.status, SwapStatusForward::Complete);
        }

        pic.advance_time(Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL));
        tick_n_blocks(pic, 3);
        let canister_owned_nfts = get_owned_nfts(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(
            canister_owned_nfts.get(&(origyn_nft, 1u16)),
            Some(&Nat::from(2u64))
        );

        let gldt_supply_post = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(gldt_supply_post, Nat::from(20_000_000_000u64));
        println!("///// gldt_supply_post : {gldt_supply_post}");

        /////////////////////////////////
        /////  Reverse Swap
        /////////////////////////////////

        // check suer has 200 gldt
        let user_balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: nft_owner.clone(),
                subaccount: None,
            },
        );
        assert_eq!(user_balance, Nat::from(20_000_000_000u64));

        let gldt_supply_pre = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        println!("///// presale supply : {gldt_supply_pre}");
        assert_eq!(gldt_supply_pre, Nat::from(20_000_000_000u64));

        // 3. pre approve the escrow transfer and verify
        let now_time = timestamp_nanos();
        icrc2_approve(
            pic,
            nft_owner,
            gldt_ledger,
            &(icrc2_approve::Args {
                from_subaccount: None,
                spender: Account {
                    owner: gldt_swap,
                    subaccount: Some(nft_id.clone().into()),
                },
                amount: Nat::from(10_100_000_000u128),
                expected_allowance: Some(Nat::from(0u64)),
                expires_at: None,
                fee: None,
                memo: None,
                created_at_time: Some(now_time),
            }),
        );
        pic.advance_time(Duration::from_millis(SECOND_IN_MS * 10));
        tick_n_blocks(pic, 2);

        // 4. start the reverse swap
        let swap_id = swap_tokens_for_nft(
            pic,
            nft_owner,
            gldt_swap,
            &(SwapTokensForNftArgs {
                nft_id: nft_id.clone(),
                nft_canister_id: origyn_nft,
            }),
        )
        .unwrap();
        matches!(swap_id, SwapId(_, _));
        tick_n_blocks(pic, 50);
        pic.advance_time(Duration::from_millis(SECOND_IN_MS * 40));
        tick_n_blocks(pic, 20);
        pic.advance_time(Duration::from_millis(HOUR_IN_MS * 3));
        pic.advance_time(Duration::from_millis(SECOND_IN_MS * 40));
        tick_n_blocks(pic, 50);

        // 5. check swap completed and is now in history
        let user_swap = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id);
        assert_eq!(&user_swap.is_some(), &true);
        if let SwapInfo::Reverse(details) = user_swap.unwrap().1 {
            assert_eq!(details.status, SwapStatusReverse::Complete);
        }
        // wait for the supply balancer to process
        pic.advance_time(Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL));
        tick_n_blocks(pic, 5);
        let canister_owned_nfts = get_owned_nfts(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(
            canister_owned_nfts.get(&(origyn_nft, 1u16)),
            Some(&Nat::from(1u64))
        );

        let gldt_supply_post = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(gldt_supply_post, Nat::from(10_000_000_000u64)); // we kept the swap fee so user has 99 and swap canister has 1
        println!("///// gldt_supply_post : {gldt_supply_post}");

        // 8. ensure escrow account is empty
        let balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(nft_id.clone().into()),
            },
        );
        assert_eq!(balance, Nat::from(0u64));

        let balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: nft_owner,
                subaccount: None,
            },
        );
        assert_eq!(balance, Nat::from(9_900_000_000u64)); // user had 200 but spent 101 to swap so has 99

        // 9. ensure fees account has correct fees
        let balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
            },
        );
        assert_eq!(balance, Nat::from(97_000_000u64)); // swap fee account should have 97 inside of it

        // 10. ensure ledger fee account kept the remaining tx fees - escrow transfer from user to escrow takes 2x and then transfer swap fee from escrow to swap fee collection
        let balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
            },
        );
        assert_eq!(balance, Nat::from(3_000_000u64)); //

        /////////////////////////////////
        /////  Forward swap
        /////////////////////////////////
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
        pic.advance_time(Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL));
        tick_n_blocks(pic, 3);
        let canister_owned_nfts = get_owned_nfts(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(
            canister_owned_nfts.get(&(origyn_nft, 1u16)),
            Some(&Nat::from(2u64))
        );
    }
}
