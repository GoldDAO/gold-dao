use std::time::Duration;

use crate::client::gldt_swap::{get_active_swaps_by_user, insert_fake_swap};
use crate::client::origyn_nft_reference::client::{
    get_token_id_as_nat, market_transfer_nft_origyn,
};
use crate::gldt_swap_suite::nft_utils;
use crate::gldt_swap_suite::{init, CanisterIds, PrincipalIds, TestEnv};
use crate::utils::tick_n_blocks;

use candid::{Nat, Principal};
use canister_time::{timestamp_millis, WEEK_IN_MS};
use gldt_swap_common::gldt::{GldtNumTokens, GldtTokenSpec};
use gldt_swap_common::nft::NftID;
use gldt_swap_common::swap::{SwapDetailForward, SwapIndex, SwapInfo, SwapStatusForward};
use icrc_ledger_types::icrc1::account::Account;
use origyn_nft_reference::origyn_nft_reference_canister::{
    Account as OrigynAccount, AskFeature, MarketTransferRequest, PricingConfigShared, SalesConfig,
};
use pocket_ic::PocketIc;

use canister_time::{MINUTE_IN_MS, SECOND_IN_MS};
use gldt_swap_common::swap::SwapErrorForward;

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

fn insert_bulk_fake_swaps(env: &mut TestEnv, num_to_insert: usize) {
    let TestEnv {
        pic,
        canister_ids:
            CanisterIds {
                origyn_nft,
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
    let time_now = timestamp_millis() + (Duration::from_millis(WEEK_IN_MS).as_millis() as u64);
    for i in 1..=num_to_insert {
        let res = init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            nft_owner.clone(),
            i.to_string(),
        );
        assert_eq!(res, true);
        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            i.to_string(),
        );
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(i),
                nft_id: NftID(token_id_as_nat),
                nft_id_string: i.to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::Init,
                sale_id: String::from(""),
                created_at: time_now,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [0u8; 32],
                gldt_receiver: Account {
                    owner: nft_owner.clone(),
                    subaccount: None,
                },
            }),
        )
        .unwrap();
    }
    let active_swaps = get_active_swaps_by_user(pic, nft_owner.clone(), gldt_swap.clone(), &None);
    assert_eq!(active_swaps.len(), num_to_insert);
}

#[cfg(test)]
mod tests {
    use gldt_swap_common::swap::SwapStatus;

    use super::*;

    #[test]
    pub fn remove_stale_swaps_bulk_works_correctly() {
        let mut env = init::init();
        let expected_active_swaps = 50usize;
        insert_bulk_fake_swaps(&mut env, expected_active_swaps);

        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ..
                },
            principal_ids: PrincipalIds { nft_owner, .. },
        } = env;

        let active_swaps =
            get_active_swaps_by_user(pic, nft_owner.clone(), gldt_swap.clone(), &None);
        assert_eq!(active_swaps.len(), expected_active_swaps);

        pic.advance_time(Duration::from_millis(WEEK_IN_MS - SECOND_IN_MS * 30));

        // pick one swap to have an active sale
        let market_args = MarketTransferRequest {
            token_id: "1".to_string(),
            sales_config: SalesConfig {
                broker_id: None,
                pricing: PricingConfigShared::Ask(Some(vec![
                    AskFeature::Token(GldtTokenSpec::new(gldt_ledger).get_token_spec()),
                    AskFeature::BuyNow(Nat::from(10_000_000_000u64)),
                    AskFeature::Notify(vec![Principal::anonymous()]),
                    AskFeature::FeeSchema("com.origyn.royalties.fixed".to_string()),
                    AskFeature::AllowList(vec![]),
                ])),
                escrow_receipt: None,
            },
        };
        market_transfer_nft_origyn(pic, origyn_nft.clone(), nft_owner, market_args);
        tick_n_blocks(pic, 1);
        pic.advance_time(Duration::from_millis(SECOND_IN_MS * 30));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 5));
        tick_n_blocks(pic, 30);

        for (swap_id, _) in active_swaps {
            let (_, swap_info) =
                get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id).unwrap();
            if let SwapInfo::Forward(detail_forward) = swap_info {
                if detail_forward.nft_id_string == "1" {
                    assert_eq!(detail_forward.status, SwapStatusForward::Init);
                } else {
                    assert_eq!(
                        detail_forward.status,
                        SwapStatusForward::Failed(SwapErrorForward::Expired(Box::new(
                            SwapStatusForward::Init
                        )))
                    );
                }
            }
        }
    }
}
