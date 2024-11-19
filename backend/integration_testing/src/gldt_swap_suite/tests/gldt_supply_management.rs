use crate::client::gldt_swap::{get_active_swaps_by_user, get_owned_nfts};
use crate::client::icrc1::icrc1_total_supply;
use crate::client::{gldt_swap::insert_fake_swap, icrc1::client::transfer};
use crate::gldt_swap_suite::nft_utils;
use crate::gldt_swap_suite::{init, CanisterIds, PrincipalIds, TestEnv};
use crate::utils::tick_n_blocks;
use candid::{Nat, Principal};
use canister_time::timestamp_millis;
use gldt_swap_common::gldt::GLDT_TX_FEE;
use gldt_swap_common::swap::NotificationError;
use gldt_swap_common::swap::MANAGE_GLDT_SUPPLY_INTERVAL;
use gldt_swap_common::swap::{SwapInfo, SwapStatusForward};
use gldt_swap_common::{
    gldt::GldtNumTokens,
    nft::NftID,
    swap::{SwapDetailForward, SwapIndex},
};
use icrc_ledger_types::icrc1::account::Account;
use init::reinstall_gldt_swap_canister;
use origyn_nft_reference::origyn_nft_reference_canister::Account as OrigynAccount;
use pocket_ic::PocketIc;
use std::time::Duration;

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
    use std::time::UNIX_EPOCH;

    use canister_time::{HOUR_IN_MS, MINUTE_IN_MS};

    use super::*;
    #[test]
    pub fn automatically_resolves_supply_imbalance() {
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
                    ..
                },
        } = env;
        tick_n_blocks(pic, 2);

        let pre_swap_gldt_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        assert_eq!(pre_swap_gldt_supply, Nat::from(0u64));
        pic.advance_time(Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL));
        tick_n_blocks(pic, 5);
        pic.stop_progress();
        pic.stop_live();
        // give the nft canister 5 x 1g NFTs
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "1".to_string(),
        );
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "2".to_string(),
        );
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "3".to_string(),
        );

        tick_n_blocks(pic, 1);
        let real_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(real_supply, Nat::from(0u64));

        pic.advance_time(Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL));
        tick_n_blocks(pic, 5);

        let expected_supply = Nat::from(30_000_000_000u64);
        let real_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        assert_eq!(real_supply, expected_supply);
    }

    #[test]
    fn will_not_balance_if_active_swap_present_and_attempt_retries() {
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

        tick_n_blocks(pic, 2);

        pic.advance_time(Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL)); // advance to when the gldt supply job starts

        // time of insert is 59 mins - after 3~ minutes it is considererd a stale swap and will be removed
        let swap_insert_time = pic
            .get_time()
            .duration_since(UNIX_EPOCH)
            .ok()
            .map(|duration| duration.as_millis())
            .unwrap();
        // insert a fake swap
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                nft_id: NftID(Nat::from(0u64)),
                nft_id_string: "1".to_string(),
                nft_canister: origyn_nft.clone(),
                status: SwapStatusForward::NotificationFailed(NotificationError::TimeoutInvalid(
                    "".to_string(),
                )),
                sale_id: "test".to_string(),
                created_at: (swap_insert_time as u64) - MINUTE_IN_MS,
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

        let active_swaps = get_active_swaps_by_user(pic, nft_owner, gldt_swap, &None);
        assert_eq!(active_swaps.len(), 1);
        tick_n_blocks(pic, 5);

        // give 3 nfts to the swap canister
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "1".to_string(),
        );
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "2".to_string(),
        );
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "3".to_string(),
        );

        tick_n_blocks(pic, 1);
        let real_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(real_supply, Nat::from(0u64));
        tick_n_blocks(pic, 5);

        // by this point the stale swap should be removed and the balance not increased because the gldt supply balancer retries with 3 minute delays
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 2)); // 1hour and 2 mins
        tick_n_blocks(pic, 5);
        let active_swaps = get_active_swaps_by_user(pic, nft_owner, gldt_swap, &None);
        assert_eq!(active_swaps.len(), 0);
        let expected_supply = Nat::from(0u64);
        let real_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(real_supply, expected_supply);

        // in one more minute the gldt supply retry will kick in
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 1));
        tick_n_blocks(pic, 5);

        let expected_supply = Nat::from(30_000_000_000u64);
        let real_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(real_supply, expected_supply);
    }

    #[test]
    fn will_not_balance_if_there_is_more_gldt_than_expected() {
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
        pic.advance_time(Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL));

        let pre_swap_gldt_supply =
            icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        assert_eq!(pre_swap_gldt_supply, Nat::from(0u64));

        // fake modify the nft count - this is analogous to doing forward swaps
        tick_n_blocks(pic, 5);
        pic.stop_progress();
        pic.stop_live();
        // give the nft canister 5 x 1g NFTs
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "1".to_string(),
        );
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "2".to_string(),
        );
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "3".to_string(),
        );

        // theortically, each forward swap should increase the supply by 100 GLDT each.
        // we will transfer 5_000_000 more ( 5 x GLDT swap fee )

        transfer(
            pic,
            gldt_swap, // minting account
            gldt_ledger,
            None,
            Account {
                owner: nft_owner,
                subaccount: None,
            },
            30_000_000_000u128 + 3 * (GLDT_TX_FEE as u128),
        )
        .unwrap();
        tick_n_blocks(pic, 1);
        let real_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(real_supply, Nat::from(30_003_000_000u64));

        // there is now an imbalance
        // there should be 50_000_000_000
        // advance time to trigger the cron job
        pic.advance_time(Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL));
        tick_n_blocks(pic, 5);

        let expected_supply = Nat::from(30_003_000_000u64);
        let real_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());

        assert_eq!(real_supply, expected_supply);
    }

    #[test]
    fn canister_should_auto_recongize_nfts_it_owns() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_ledger,
                    gldt_swap,
                    ogy_ledger,
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
        tick_n_blocks(pic, 2);
        // create the new nfts and give them to the swap canister
        pic.stop_canister(gldt_swap, Some(controller)).unwrap();

        // reinstall_gldt_swap_canister
        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "1".to_string(),
        );

        init_nft_with_premint_nft(
            pic,
            origyn_nft.clone(),
            originator.clone(),
            net_principal.clone(),
            gldt_swap.clone(),
            "2".to_string(),
        );
        tick_n_blocks(pic, 3);

        let real_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(real_supply, Nat::from(0u64));

        // give some gldt to the user, but give them a bad amount ( under what would be expected for 2 nfts locked in the swap canister)
        transfer(
            pic,
            gldt_swap,
            gldt_ledger,
            None,
            Account {
                owner: nft_owner,
                subaccount: None,
            },
            19_900_000_000u128, // we intentionally minus 2 transaction fees because
        )
        .unwrap();

        let real_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(real_supply, Nat::from(19_900_000_000u64));

        // lets reinstall the swap canister to trigger the initial cron job again
        reinstall_gldt_swap_canister(
            pic,
            &controller,
            gldt_swap,
            gldt_ledger,
            ogy_ledger,
            origyn_nft,
        );

        tick_n_blocks(pic, 20);
        pic.advance_time(Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL));
        tick_n_blocks(pic, 20);

        let owned_nfts = get_owned_nfts(pic, Principal::anonymous(), gldt_swap, &());
        println!("{owned_nfts:?}");
        assert_eq!(
            owned_nfts.get(&(origyn_nft, 1u16)).unwrap(),
            &Nat::from(2u64)
        );

        // important - check that the supply of gldt is correct
        let real_supply = icrc1_total_supply(pic, Principal::anonymous(), gldt_ledger, &());
        assert_eq!(real_supply, Nat::from(20_000_000_000u64));
    }
}
