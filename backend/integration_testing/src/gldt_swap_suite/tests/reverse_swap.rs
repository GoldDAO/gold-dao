use std::time::Duration;

use crate::client::gldt_swap::{
    get_active_swaps_by_user, insert_fake_swap, recover_stuck_swap, swap_tokens_for_nft,
};
use crate::client::icrc1::client::{balance_of, transfer};
use crate::client::icrc1::icrc1_total_supply;
use crate::client::icrc1_icrc2_token::{icrc2_allowance, icrc2_approve};
use crate::client::origyn_nft_reference::client::{get_token_id_as_nat, icrc7_owner_of};
use crate::gldt_swap_suite::nft_utils;
use crate::gldt_swap_suite::{init, CanisterIds, PrincipalIds, TestEnv};
use crate::utils::tick_n_blocks;

use canister_time::{timestamp_millis, timestamp_nanos, MINUTE_IN_MS, SECOND_IN_MS};
use gldt_swap_api_canister::swap_tokens_for_nft::Args;
use gldt_swap_common::gldt::{GldtNumTokens, GLDT_SWAP_FEE_ACCOUNT, GLDT_TX_FEE};

use candid::{Nat, Principal};
use gldt_swap_common::nft::NftID;
use gldt_swap_common::swap::{
    NftTransferError, SwapDetailReverse, SwapErrorReverse, SwapInfo, SwapStatusReverse,
};
use icrc_ledger_types::icrc1::account::Account;
use origyn_nft_reference::origyn_nft_reference_canister::Account as OrigynAccount;
use pocket_ic::PocketIc;

use gldt_swap_common::swap::{SwapId, SwapIndex};

use crate::client::gldt_swap::get_swap;
use gldt_swap_api_canister::swap_tokens_for_nft::SwapTokensForNftRequestErrors;
use gldt_swap_common::swap::{ServiceDownReason, STALE_SWAP_TIME_THRESHOLD_MINUTES};
use origyn_nft_reference::origyn_nft_reference_canister::{
    SaleInfoRequest, SaleInfoResponse, SaleInfoResult,
};

use crate::client::{
    gldt_swap::force_toggle_gldt_supply_cron, icrc1::icrc1_transfer,
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
    use super::*;

    #[test]
    pub fn reverse_swap_should_fail_if_user_has_incorrect_gldt_balance() {
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
        tick_n_blocks(pic, 10);
        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );
        let nft_id = NftID(token_id_as_nat.clone());

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

        let res = swap_tokens_for_nft(
            pic,
            nft_owner,
            gldt_swap,
            &(Args {
                nft_id: nft_id.clone(),
                nft_canister_id: origyn_nft,
            }),
        );
        matches!(
            res,
            Err(SwapTokensForNftRequestErrors::NftValidationErrors(_))
        );
    }

    #[test]
    pub fn reverse_swap_should_fail_if_gldt_supply_cron_is_running() {
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
                    ..
                },
        } = env;
        tick_n_blocks(pic, 10);
        // 1. setup nft and verify owner
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );
        let nft_id = NftID(token_id_as_nat.clone());

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

        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: nft_owner,
                subaccount: None,
            },
            10_100_000_000u128,
        )
        .unwrap();
        let balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: nft_owner,
                subaccount: None,
            },
        );
        assert_eq!(balance, Nat::from(10_100_000_000u128));

        force_toggle_gldt_supply_cron(pic, controller, gldt_swap, &());
        tick_n_blocks(pic, 1);

        let res = swap_tokens_for_nft(
            pic,
            nft_owner,
            gldt_swap,
            &(Args {
                nft_id: nft_id.clone(),
                nft_canister_id: origyn_nft,
            }),
        );
        match res {
            Ok(_) => {
                panic!("should have errored");
            }
            Err(e) => {
                matches!(e, SwapTokensForNftRequestErrors::Retry(_));
            }
        }

        force_toggle_gldt_supply_cron(pic, controller, gldt_swap, &());
        tick_n_blocks(pic, 2);

        let res = swap_tokens_for_nft(
            pic,
            nft_owner,
            gldt_swap,
            &(Args {
                nft_id: nft_id.clone(),
                nft_canister_id: origyn_nft,
            }),
        );
        println!("{res:?}");
        assert_eq!(res.is_ok(), true)
    }

    #[test]
    pub fn reverse_swap_will_fail_if_there_is_not_enough_ogy() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    ogy_ledger,
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
            gldt_swap.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );
        let nft_id = NftID(token_id_as_nat.clone());

        tick_n_blocks(pic, 3);

        // 2. give some gldt to userf
        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: nft_owner,
                subaccount: None,
            },
            10_100_000_000u128,
        )
        .unwrap();
        let balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: nft_owner,
                subaccount: None,
            },
        );
        assert_eq!(balance, Nat::from(10_100_000_000u128));

        ////////////////////////
        /////  remove all OGY from nft fee account
        //////////////////////

        let info_req = sale_info_nft_origyn(
            pic,
            gldt_swap,
            origyn_nft,
            &SaleInfoRequest::FeeDepositInfo(Some(OrigynAccount::Account {
                owner: gldt_swap,
                sub_account: None,
            })),
        );

        let account = match info_req {
            SaleInfoResult::Ok(ok_res) => match ok_res {
                SaleInfoResponse::FeeDepositInfo(fee_deposit_info) => {
                    let account = Account {
                        owner: fee_deposit_info.account.principal,
                        subaccount: Some(
                            fee_deposit_info
                                .account
                                .sub_account
                                .as_slice()
                                .try_into()
                                .unwrap(),
                        ),
                    };
                    account
                }
                _ => {
                    panic!("Can't find account")
                }
            },
            SaleInfoResult::Err(error) => {
                panic!("Can't find account {error:?}")
            }
        };

        let starting_ogy_balance = balance_of(pic, ogy_ledger, account);
        println!("{starting_ogy_balance:?}");
        assert_eq!(starting_ogy_balance, Nat::from(1_000_000_000_000u64)); // starting fee balance

        let transfer_amount = starting_ogy_balance - Nat::from(200_000u64);
        // reduce the balance
        let dummy_account = Account {
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
                to: dummy_account,
                fee: None,
                created_at_time: None,
                memo: None,
                amount: transfer_amount.clone(),
            }),
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
        assert_eq!(res, Nat::from(0u64));

        // 4. pre approve the escrow transfer
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
                created_at_time: Some(timestamp_nanos()),
            }),
        );
        pic.advance_time(Duration::from_millis(SECOND_IN_MS * 10));
        tick_n_blocks(pic, 2);

        let allowance = icrc2_allowance(
            pic,
            Principal::anonymous(),
            gldt_ledger,
            &(icrc2_allowance::Args {
                account: Account {
                    owner: nft_owner,
                    subaccount: None,
                },
                spender: Account {
                    owner: gldt_swap,
                    subaccount: Some(nft_id.clone().into()),
                },
            }),
        );
        assert_eq!(allowance.allowance, Nat::from(10_100_000_000u128));

        // 5. start the reverse swap
        let swap_id = swap_tokens_for_nft(
            pic,
            nft_owner,
            gldt_swap,
            &(Args {
                nft_id: nft_id.clone(),
                nft_canister_id: origyn_nft,
            }),
        )
        .err();
        matches!(
            swap_id,
            Some(SwapTokensForNftRequestErrors::ServiceDown(
                ServiceDownReason::LowOrigynToken(_)
            ))
        );
        tick_n_blocks(pic, 90);
    }

    #[test]
    pub fn reverse_swap_will_refund_if_failed() {
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
            gldt_swap.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );
        let nft_id = NftID(token_id_as_nat.clone());

        tick_n_blocks(pic, 3);

        // 2. insert a failed swap
        insert_fake_swap(
            pic,
            controller,
            gldt_swap,
            &SwapInfo::Reverse(SwapDetailReverse {
                index: SwapIndex::from(0u64),
                nft_id: nft_id.clone(),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft,
                status: SwapStatusReverse::NftTransferFailed(NftTransferError::CallError(
                    "something went wrong".to_string(),
                )),
                created_at: timestamp_millis(),
                tokens_to_receive: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                swap_fee: Nat::from(100_000_000u64),
                transfer_fees: Nat::from(GLDT_TX_FEE * 2),
                user: nft_owner,
            }),
        )
        .unwrap();

        // 3. mint some gldt on behalf of the user to the escrow account
        let res = transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: gldt_swap,
                subaccount: Some(nft_id.clone().into()),
            },
            10_100_000_000u128 - (2 * GLDT_TX_FEE) as u128, // we intentionally minus 2 transaction fees because
        )
        .unwrap();
        tick_n_blocks(pic, 3);
        println!("transfer to escrow success {res}");

        // 4. call the recovery method
        let user_swaps = get_active_swaps_by_user(pic, nft_owner, gldt_swap, &Some(nft_owner));
        assert_eq!(&user_swaps.len(), &1usize);
        let swap_id = &user_swaps[0].0;
        // wait for 3 minutes for the swap to become stale
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);
        let res = recover_stuck_swap(pic, controller, gldt_swap, &swap_id).unwrap();
        matches!(res, SwapId(_, _));
        tick_n_blocks(pic, 5);

        // 5. swap should be in refunded state
        let user_swap = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id);
        assert_eq!(&user_swap.is_some(), &true);
        if let SwapInfo::Reverse(details) = user_swap.unwrap().1 {
            assert_eq!(
                details.status,
                SwapStatusReverse::Failed(SwapErrorReverse::Refunded(Box::new(
                    SwapStatusReverse::NftTransferFailed(NftTransferError::CallError(
                        "something went wrong".to_string()
                    ))
                )))
            );
        } else {
            panic!("Forward swap returned but should be reverse swap");
        }
        // 6. active swaps should be 0
        let user_swaps = get_active_swaps_by_user(pic, nft_owner, gldt_swap, &Some(nft_owner));
        assert_eq!(&user_swaps.len(), &0usize);

        // 7. ensure escrow is empty and the user has their gldt refunded
        let balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(nft_id.clone().into()),
            },
        );
        assert_eq!(balance, Nat::from(0u64));

        // ensure user has their gldt
        let balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: nft_owner,
                subaccount: None,
            },
        );
        assert_eq!(
            balance,
            Nat::from(10_100_000_000u128 - (GLDT_TX_FEE as u128) * 3)
        );

        // 8. ensure swap canister still owns the nft
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
    pub fn reverse_swap_recover_stuck_burn() {
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
            gldt_swap.clone(),
            "1".to_string(),
        );

        let token_id_as_nat = get_token_id_as_nat(
            pic,
            origyn_nft.clone(),
            net_principal.clone(),
            "1".to_string(),
        );
        let nft_id = NftID(token_id_as_nat.clone());

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
        tick_n_blocks(pic, 3);

        // 2. mint some gldt to the escrow account
        let res = transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: gldt_swap,
                subaccount: Some(nft_id.clone().into()),
            },
            10_100_000_000u128 - (2 * GLDT_TX_FEE) as u128, // we intentionally minus 2 transaction fees because
        )
        .unwrap();
        tick_n_blocks(pic, 3);
        println!("transfer to escrow success {res}");
        let pre_gldt_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        tick_n_blocks(pic, 10);
        // let swap_id = SwapId(nft_id.clone(), SwapIndex::from(0u64));
        // 3. insert the fake swap ( just after nft transfer is successful )
        let res = insert_fake_swap(
            pic,
            controller,
            gldt_swap,
            &SwapInfo::Reverse(SwapDetailReverse {
                index: SwapIndex::from(0u64),
                nft_id: nft_id.clone(),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft,
                status: SwapStatusReverse::BurnRequest,
                created_at: timestamp_millis(),
                tokens_to_receive: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                swap_fee: Nat::from(100_000_000u64),
                transfer_fees: Nat::from(GLDT_TX_FEE * 2),
                user: nft_owner,
            }),
        )
        .unwrap();
        tick_n_blocks(pic, 5);
        assert_eq!(res, ());

        // 5. check stuck swap got inserted correctly and not into history
        let user_swaps = get_active_swaps_by_user(pic, nft_owner, gldt_swap, &Some(nft_owner));
        assert_eq!(&user_swaps.len(), &1usize);
        let swap_id = &user_swaps[0].0;

        if let SwapInfo::Reverse(details) = user_swaps[0].clone().1 {
            assert_eq!(details.status, SwapStatusReverse::BurnRequest);
        }

        // 6. call the recovery method
        // wait for 5 minutes
        pic.advance_time(Duration::from_millis(
            MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES,
        ));
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 6);
        let res = recover_stuck_swap(pic, controller, gldt_swap, &swap_id).unwrap();

        matches!(res, SwapId(_, _));
        tick_n_blocks(pic, 60);

        // 7. swap should be completed
        let user_swap = get_swap(pic, Principal::anonymous(), gldt_swap, &swap_id);
        assert_eq!(&user_swap.is_some(), &true);
        if let SwapInfo::Reverse(details) = user_swap.unwrap().1 {
            assert_eq!(details.status, SwapStatusReverse::Complete);
        }

        // 7. ensure supply of gldt is lower
        let post_sale_total_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        let expected_supply = pre_gldt_supply - Nat::from(10_000_000_000u64); // at this point, there is only 1 transaction that occurs ( swap fee -> fee sub account pool)
        assert_eq!(post_sale_total_supply, expected_supply);

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

        // 9. ensure fees account has correct fees
        let balance = balance_of(
            pic,
            gldt_ledger,
            Account {
                owner: gldt_swap,
                subaccount: Some(GLDT_SWAP_FEE_ACCOUNT),
            },
        );
        assert_eq!(balance, Nat::from(70_000_000u64));
    }
}
