use std::time::Duration;
use canister_time::HOUR_IN_MS;
use gldt_swap_common::{ archive::ArchiveCanister, swap::SwapIndex };

use crate::client::gldt_swap::get_archive_canisters;
use crate::client::gldt_swap::insert_fake_bulk_swaps;
use crate::client::gldt_swap::insert_fake_swap;
use crate::gldt_swap_suite::{ init, CanisterIds, PrincipalIds, TestEnv };
use crate::utils::tick_n_blocks;

use canister_time::{ timestamp_millis, WEEK_IN_MS };
use gldt_swap_common::gldt::GldtNumTokens;
use gldt_swap_common::nft::NftID;
use gldt_swap_common::swap::{ SwapDetailForward, SwapErrorForward, SwapInfo, SwapStatusForward };
use icrc_ledger_types::icrc1::account::Account;
use candid::{ Nat, Principal };
use pocket_ic::PocketIc;

fn insert_bulk_fake_swaps(
    pic: &mut PocketIc,
    num_to_insert: usize,
    controller: Principal,
    gldt_swap: Principal
) -> (Principal, Principal) {
    let user_a = Principal::from_slice(
        &[
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
        ]
    );
    let user_b = Principal::from_slice(
        &[
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8,
        ]
    );
    let mut swaps: Vec<SwapInfo> = vec![];
    for i in 0..num_to_insert {
        let user = if i == 0 { user_a } else if i % 2 == 0 { user_b } else { user_a };
        let time_now = timestamp_millis() + (Duration::from_millis(WEEK_IN_MS).as_millis() as u64);
        swaps.push(
            SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(i),
                nft_id: NftID(Nat::from(i)),
                nft_id_string: i.to_string(),
                nft_canister: Principal::anonymous(),
                status: SwapStatusForward::Failed(SwapErrorForward::Expired),
                sale_id: String::from(""),
                created_at: time_now,
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: [0u8; 32],
                gldt_receiver: Account { owner: user, subaccount: None },
            })
        );
    }
    insert_fake_bulk_swaps(pic, controller.clone(), gldt_swap.clone(), &swaps).unwrap();
    pic.advance_time(Duration::from_millis(1000));
    pic.tick();
    (user_a, user_b)
}

#[cfg(test)]
mod tests {
    use canister_time::MINUTE_IN_MS;

    use super::*;
    #[test]
    pub fn gldt_swap_manages_archive_canister_cycles_happy_path() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids: CanisterIds { gldt_swap, .. },
            principal_ids: PrincipalIds { controller, .. },
        } = env;
        tick_n_blocks(pic, 2); // need to wait for cron job to finish creating the archive

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 1);

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 1);
        let ArchiveCanister { canister_id: first_archive_principal, .. } = archive_canisters
            .first()
            .unwrap();
        let swap_canister_cycle_balance = pic.cycle_balance(gldt_swap);
        let first_archive_cycle_balance_before_swaps = pic.cycle_balance(
            first_archive_principal.clone()
        );
        println!("//// swap canister cycle balance  : {swap_canister_cycle_balance:?}");
        println!(
            "//// first archive cycle balance  : {first_archive_cycle_balance_before_swaps:?}"
        );

        // In test mode there is a threshhold of approximately 32mb memory and a max swap size of 28500 ( this affects how many we can store before the btreemap is allocated more memory)
        // 366 reprensts the amount of swaps needed to create 2 archive canisters.
        let _ = insert_bulk_fake_swaps(pic, 366, controller, gldt_swap);
        // let _ = insert_bulk_fake_swaps(pic, 1087, controller, gldt_swap);
        tick_n_blocks(pic, 10);
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 10);
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
        tick_n_blocks(pic, 10);
        println!("//// swaps inserted ");
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 2);
        println!("/// all archive canisters {archive_canisters:?}");
        let ArchiveCanister { canister_id: second_archive_principal, .. } = archive_canisters
            .last()
            .unwrap();

        let swap_canister_cycle_balance_after_swaps = pic.cycle_balance(gldt_swap);
        let first_archive_cycle_balance_after_swaps = pic.cycle_balance(
            first_archive_principal.clone()
        );
        let second_canister_cycle_balance = pic.cycle_balance(second_archive_principal.clone());

        println!("//// after swap : swap canister  : {swap_canister_cycle_balance_after_swaps:?}");
        println!("//// after swap : first archive  : {first_archive_cycle_balance_after_swaps:?}");
        println!("//// after swap : second archive : {second_canister_cycle_balance:?}");

        pic.advance_time(Duration::from_millis(HOUR_IN_MS));
        tick_n_blocks(pic, 2);
        pic.advance_time(Duration::from_millis(HOUR_IN_MS));
        tick_n_blocks(pic, 2);

        println!("//// cron job time passed ");

        let swap_canister_cycle_balance_after_cron = pic.cycle_balance(gldt_swap);
        let first_archive_cycle_balance_after_cron = pic.cycle_balance(
            first_archive_principal.clone()
        );
        let second_canister_cycle_balance_after_cron = pic.cycle_balance(
            second_archive_principal.clone()
        );

        println!(
            "//// after 2 hours : swap canister  : {swap_canister_cycle_balance_after_cron:?}"
        );
        println!(
            "//// after 2 hours : first archive    : {first_archive_cycle_balance_after_cron:?}"
        );
        println!(
            "//// after 2 hours : third archive: {second_canister_cycle_balance_after_cron:?}"
        );

        assert!(first_archive_cycle_balance_after_cron > first_archive_cycle_balance_after_swaps);
        assert!(second_canister_cycle_balance_after_cron > second_canister_cycle_balance);
    }
}

// 1,896,740,496,662
// 1,895,905,182,909
