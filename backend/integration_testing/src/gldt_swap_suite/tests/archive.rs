use std::time::Duration;

use crate::client::gldt_swap::{ get_swap, insert_fake_swap };
use crate::gldt_swap_suite::{ init, CanisterIds, PrincipalIds, TestEnv };
use crate::utils::tick_n_blocks;

use canister_time::{ timestamp_millis, HOUR_IN_MS, WEEK_IN_MS };
use gldt_swap_common::gldt::GldtNumTokens;
use gldt_swap_common::nft::NftID;
use gldt_swap_common::swap::{
    SwapDetailForward,
    SwapErrorForward,
    SwapIndex,
    SwapInfo,
    SwapStatusForward,
};
use icrc_ledger_types::icrc1::account::Account;
use candid::{ Nat, Principal };
use pocket_ic::PocketIc;

fn insert_bulk_fake_swaps(
    pic: &mut PocketIc,
    num_to_insert: usize,
    controller: Principal,
    gldt_swap: Principal
) -> (Principal, Principal) {
    let time_now = timestamp_millis() + (Duration::from_millis(WEEK_IN_MS).as_millis() as u64);
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
    for i in 0..num_to_insert {
        let user = if i == 0 { user_a } else if i % 2 == 0 { user_b } else { user_a };
        insert_fake_swap(
            pic,
            controller.clone(),
            gldt_swap.clone(),
            &SwapInfo::Forward(SwapDetailForward {
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
        ).unwrap();
        pic.advance_time(Duration::from_millis(HOUR_IN_MS));
        pic.tick();
    }
    (user_a, user_b)
}

#[cfg(test)]
mod tests {
    use candid::Encode;
    use gldt_swap_common::{ archive::ArchiveCanister, nft::NftCanisterConf, swap::SwapId };

    use gldt_swap_api_canister::{
        get_historic_swaps::{ Args as GetHistoricSwapsArgs, GetHistoricSwapsError },
        get_historic_swaps_by_user::Args as GetHistoricSwapsByUserArgs,
        lifecycle::Args as GldtSwapCanisterArgs,
        init::InitArgs as GldtSwapCanisterInitArgs,
        post_upgrade::UpgradeArgs as GldtSwapCanisterUpgradeArgs,
    };
    use gldt_swap_api_archive::get_archive_swaps::{
        Args as GetArchiveSwapArgs,
        Response as GetArchiveSwapResponse,
    };
    use types::BuildVersion;

    #[test]
    pub fn init_should_create_a_default_archive() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids: CanisterIds { gldt_swap, .. },
            principal_ids: PrincipalIds { .. },
        } = env;
        tick_n_blocks(pic, 2); // need to wait for cron job to finish creating the archive
        // note a cron job runs when the canister wasm is installed and it auto creates a new archive canister
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 1);
    }

    use crate::{
        client::gldt_swap::{
            get_archive_canisters,
            get_archive_swaps,
            get_historic_swaps,
            get_historic_swaps_by_user,
            get_history_total,
            get_version,
        },
        wasms,
    };

    use super::*;
    #[test]
    pub fn archive_features_work_correctly() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids: CanisterIds { origyn_nft, gldt_swap, gldt_ledger, ogy_ledger, .. },
            principal_ids: PrincipalIds { controller, .. },
        } = env;
        tick_n_blocks(pic, 2); // need to wait for cron job to finish creating the archive
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 1);

        // In test mode there is a threshhold of approximately 10MB before a new archive canister is created.
        let (user_a, _) = insert_bulk_fake_swaps(pic, 250, controller, gldt_swap);

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 2);
        let ArchiveCanister { start_index: second_archive_start_index, .. } = archive_canisters
            .last()
            .unwrap();
        println!("archive canister 2 index : {second_archive_start_index:?}");
        println!("archive canisters : {archive_canisters:?}");
        let total_swaps: Nat = get_history_total(pic, Principal::anonymous(), gldt_swap, &None);
        assert_eq!(total_swaps, Nat::from(250u64));
        let mut start_swap_index = 0usize;

        // test all individual swaps are locatable first with no duplicates or extra swaps
        for i in 0..250u64 {
            let res = get_swap(
                pic,
                Principal::anonymous(),
                gldt_swap,
                &SwapId(NftID(Nat::from(i)), Nat::from(i))
            ).unwrap();
            assert_eq!(res.0.1, Nat::from(i));
        }
        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(250u64)), Nat::from(250u64))
        );
        assert_eq!(res.is_none(), true);

        // the starting index should be retrievable
        let swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(
                NftID(second_archive_start_index.clone()),
                Nat::from(second_archive_start_index.clone())
            )
        );
        assert_eq!(swap.is_some(), true);

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        // there should be 2 archive canisters
        assert_eq!(archive_canisters.len(), 2);

        let archive_canister_1 = archive_canisters[0].clone();
        let archive_canister_2 = archive_canisters[1].clone();

        println!("{archive_canister_1:?}");
        println!("{archive_canister_2:?}");

        // start at 249 ( the latest swap ) and go to the oldest swap
        let mut start_index = total_swaps.clone(); // starts at 249
        println!("///// start_index : {start_index:?}");
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_2.canister_id,
            &(GetArchiveSwapArgs {
                start: start_index.clone(),
                limit: 50,
                user_principal: None,
            })
        );
        assert_eq!(res.len(), 50);
        for (swap, expected_id) in res.iter().zip((0..50).rev().map(|i| Nat::from(i + 200u64))) {
            assert_eq!(expected_id, swap.0.1);
        }
        assert_eq!(res[0].0.1, Nat::from(249u64));
        assert_eq!(res[49].0.1, Nat::from(200u64));

        start_index = res[49].0.1.clone() - Nat::from(1u64);
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_2.canister_id,
            &(GetArchiveSwapArgs {
                start: start_index.clone(),
                limit: 50,
                user_principal: None,
            })
        );
        assert_eq!(res.len(), 19);
        assert_eq!(res[0].0.1, Nat::from(199u64));
        assert_eq!(res[18].0.1, Nat::from(181u64));

        // using archive canister 1 now because we've gone through all of archive canister 2's records
        start_index = res[18].0.1.clone() - Nat::from(1u64);
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_1.canister_id,
            &(GetArchiveSwapArgs {
                start: start_index.clone(),
                limit: 50,
                user_principal: None,
            })
        );
        assert_eq!(res.len(), 50);
        assert_eq!(res[0].0.1, Nat::from(180u64));
        assert_eq!(res[49].0.1, Nat::from(131u64));

        start_index = res[49].0.1.clone() - Nat::from(1u64);
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_1.canister_id,
            &(GetArchiveSwapArgs {
                start: start_index.clone(),
                limit: 50,
                user_principal: None,
            })
        );
        assert_eq!(res.len(), 50);
        assert_eq!(res[0].0.1, Nat::from(130u64));
        assert_eq!(res[49].0.1, Nat::from(81u64));

        start_index = res[49].0.1.clone() - Nat::from(1u64);
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_1.canister_id,
            &(GetArchiveSwapArgs {
                start: start_index.clone(),
                limit: 50,
                user_principal: None,
            })
        );
        assert_eq!(res.len(), 50);
        assert_eq!(res[0].0.1, Nat::from(80u64));
        assert_eq!(res[49].0.1, Nat::from(31u64));

        start_index = res[49].0.1.clone() - Nat::from(1u64);
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_1.canister_id,
            &(GetArchiveSwapArgs {
                start: start_index.clone(),
                limit: 50,
                user_principal: None,
            })
        );
        assert_eq!(res.len(), 31);
        assert_eq!(res[0].0.1, Nat::from(30u64));
        assert_eq!(res[30].0.1, Nat::from(0u64));

        // test getting user historic swaps
        let total_user_swaps = get_history_total(pic, user_a, gldt_swap, &Some(user_a));
        let mut running_total = 0u64;

        start_index = Nat::from(250u64);
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_2.canister_id,
            &(GetArchiveSwapArgs {
                start: start_index,
                limit: 50,
                user_principal: Some(user_a),
            })
        );
        assert_eq!(res.len(), 35);
        running_total += 35;

        start_index = res[34].0.1.clone() - Nat::from(1u64);
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_1.canister_id,
            &(GetArchiveSwapArgs {
                start: start_index,
                limit: 50,
                user_principal: Some(user_a),
            })
        );
        assert_eq!(res.len(), 50);
        running_total += 50;

        start_index = res[49].0.1.clone() - Nat::from(1u64);
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_1.canister_id,
            &(GetArchiveSwapArgs {
                start: start_index,
                limit: 50,
                user_principal: Some(user_a),
            })
        );
        assert_eq!(res.len(), 41);
        running_total += 41;

        assert_eq!(total_user_swaps, Nat::from(running_total));

        // will upgrade multiple canisters
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        for archive in archive_canisters {
            let version = get_version(
                pic,
                Principal::anonymous(),
                archive.canister_id.clone(),
                &()
            );
            assert_eq!(version, BuildVersion::new(0, 0, 0));
        }

        // upgrading should work fine
        let gldt_swap_canister_wasm: Vec<u8> = wasms::GLDT_SWAP.clone();
        let gldt_swap_init_args = Encode!(
            &GldtSwapCanisterArgs::Upgrade(GldtSwapCanisterUpgradeArgs {
                version: BuildVersion::new(0, 0, 2),
                commit_hash: "zyxwvut".to_string(),
            })
        ).unwrap();
        pic.upgrade_canister(
            gldt_swap,
            gldt_swap_canister_wasm,
            gldt_swap_init_args,
            Some(controller)
        ).unwrap();
        tick_n_blocks(pic, 20);

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        for archive in archive_canisters {
            let version = get_version(
                pic,
                Principal::anonymous(),
                archive.canister_id.clone(),
                &()
            );
            assert_eq!(version, BuildVersion::new(0, 0, 2));
        }

        // get a swap from both archive canisters
        get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(50u64)), Nat::from(50u64))
        ).unwrap();

        get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(210u64)), Nat::from(210u64))
        ).unwrap();

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 2);
    }

    #[test]
    pub fn get_historic_swaps_works_correctly() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids: CanisterIds { origyn_nft, gldt_swap, gldt_ledger, ogy_ledger, .. },
            principal_ids: PrincipalIds { controller, .. },
        } = env;
        tick_n_blocks(pic, 2); // need to wait for cron job to finish creating the archive
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 1);

        // In test mode there is a threshhold of approximately 10MB before a new archive canister is created.
        let (user_a, _) = insert_bulk_fake_swaps(pic, 250, controller, gldt_swap);

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 2);
        let ArchiveCanister { start_index: second_archive_start_index, .. } = archive_canisters
            .last()
            .unwrap();
        println!("archive canister 2 index : {second_archive_start_index:?}");
        println!("archive canisters : {archive_canisters:?}");
        let total_swaps: Nat = get_history_total(pic, Principal::anonymous(), gldt_swap, &None);
        assert_eq!(total_swaps, Nat::from(250u64));
        let mut start_swap_index = 0usize;

        // test all individual swaps are locatable first with no duplicates or extra swaps
        for i in 0..250u64 {
            let res = get_swap(
                pic,
                Principal::anonymous(),
                gldt_swap,
                &SwapId(NftID(Nat::from(i)), Nat::from(i))
            ).unwrap();
            assert_eq!(res.0.1, Nat::from(i));
        }
        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(250u64)), Nat::from(250u64))
        );
        assert_eq!(res.is_none(), true);

        // the starting index should be retrievable
        let swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(
                NftID(second_archive_start_index.clone()),
                Nat::from(second_archive_start_index.clone())
            )
        );
        assert_eq!(swap.is_some(), true);

        // test simple pagination - should be able to go all the way from 249 to 0
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: start_swap_index,
                limit: 50,
            })
        ).unwrap();
        assert_eq!(res.len(), 50);
        for (swap, expected_id) in res.iter().zip((0..50).rev().map(|i| Nat::from(i + 200u64))) {
            assert_eq!(expected_id, swap.0.1);
        }

        start_swap_index += 1;
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: start_swap_index,
                limit: 50,
            })
        ).unwrap();
        assert_eq!(res.len(), 50);
        for (swap, expected_id) in res.iter().zip((0..50).rev().map(|i| Nat::from(i + 150u64))) {
            assert_eq!(expected_id, swap.0.1);
        }

        start_swap_index += 1;
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: start_swap_index,
                limit: 50,
            })
        ).unwrap();
        assert_eq!(res.len(), 50);
        for (swap, expected_id) in res.iter().zip((0..50).rev().map(|i| Nat::from(i + 100u64))) {
            assert_eq!(expected_id, swap.0.1);
        }

        start_swap_index += 1;
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: start_swap_index,
                limit: 50,
            })
        ).unwrap();
        assert_eq!(res.len(), 50);
        for (swap, expected_id) in res.iter().zip((0..50).rev().map(|i| Nat::from(i + 50u64))) {
            assert_eq!(expected_id, swap.0.1);
        }

        start_swap_index += 1;
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: start_swap_index,
                limit: 50,
            })
        ).unwrap();
        assert_eq!(res.len(), 50);
        for (swap, expected_id) in res.iter().zip((0..50u64).rev()) {
            assert_eq!(Nat::from(expected_id), swap.0.1);
        }

        // test a limit that is too large
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: start_swap_index,
                limit: 201,
            })
        );
        matches!(res, Err(GetHistoricSwapsError::LimitTooLarge(_)));

        // // test when start + limit is more than the total number of swaps

        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: 2, // 300
                limit: 100,
            })
        ).unwrap();
        assert_eq!(res.len(), 50);
        for (swap, expected_id) in res.iter().zip((0..50u64).rev()) {
            assert_eq!(Nat::from(expected_id), swap.0.1);
        }
        assert_eq!(res.last().unwrap().0.1, Nat::from(0u64));

        // test a page that should return no swaps
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: 3, // start at 400
                limit: 100,
            })
        ).unwrap();
        if &res.len() > &0usize {
            let last_swap = res.get(0);
            println!("{last_swap:?}");
        }
        assert_eq!(res.len(), 0);

        // test getting user historic swaps
        let res = get_historic_swaps_by_user(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsByUserArgs {
                page: 0,
                limit: 50,
                user: user_a,
            })
        ).unwrap();
        assert_eq!(res.len(), 50);
        assert_eq!(res.first().unwrap().0.1, Nat::from(249u64));
        assert_eq!(res.last().unwrap().0.1, Nat::from(151u64));

        let res = get_historic_swaps_by_user(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsByUserArgs {
                page: 1,
                limit: 50,
                user: user_a,
            })
        ).unwrap();
        assert_eq!(res.len(), 50);
        assert_eq!(res.first().unwrap().0.1, Nat::from(149u64));
        assert_eq!(res.last().unwrap().0.1, Nat::from(51u64));

        let res = get_historic_swaps_by_user(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsByUserArgs {
                page: 2,
                limit: 50,
                user: user_a,
            })
        ).unwrap();
        assert_eq!(res.len(), 26);
        assert_eq!(res.first().unwrap().0.1, Nat::from(49u64));
        assert_eq!(res.last().unwrap().0.1, Nat::from(0u64));

        // check user_history_total works
        let user_history_total = get_history_total(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &Some(user_a)
        );
        assert_eq!(user_history_total, Nat::from(126u64));

        // will upgrade multiple canisters
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        for archive in archive_canisters {
            let version = get_version(
                pic,
                Principal::anonymous(),
                archive.canister_id.clone(),
                &()
            );
            assert_eq!(version, BuildVersion::new(0, 0, 0));
        }

        // upgrading should work fine
        let gldt_swap_canister_wasm: Vec<u8> = wasms::GLDT_SWAP.clone();
        let gldt_swap_init_args = Encode!(
            &GldtSwapCanisterArgs::Upgrade(GldtSwapCanisterUpgradeArgs {
                version: BuildVersion::new(0, 0, 2),
                commit_hash: "zyxwvut".to_string(),
            })
        ).unwrap();
        pic.upgrade_canister(
            gldt_swap,
            gldt_swap_canister_wasm,
            gldt_swap_init_args,
            Some(controller)
        ).unwrap();
        tick_n_blocks(pic, 20);

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        for archive in archive_canisters {
            let version = get_version(
                pic,
                Principal::anonymous(),
                archive.canister_id.clone(),
                &()
            );
            assert_eq!(version, BuildVersion::new(0, 0, 2));
        }

        // get a swap from both archive canisters
        get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(50u64)), Nat::from(50u64))
        ).unwrap();

        get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(210u64)), Nat::from(210u64))
        ).unwrap();

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 2);
    }

    #[test]
    fn archive_will_upgrade() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids: CanisterIds { origyn_nft, gldt_swap, gldt_ledger, ogy_ledger, .. },
            principal_ids: PrincipalIds { controller, .. },
        } = env;
        tick_n_blocks(pic, 5);

        // get the first archive canister
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 1);
        let archive_canister = archive_canisters[0].clone();

        let version = get_version(
            pic,
            Principal::anonymous(),
            archive_canister.canister_id.clone(),
            &()
        );
        assert_eq!(version, BuildVersion::new(0, 0, 0));

        // upgrading should work fine
        let gldt_swap_canister_wasm: Vec<u8> = wasms::GLDT_SWAP.clone();
        let gldt_swap_init_args = Encode!(
            &GldtSwapCanisterArgs::Upgrade(GldtSwapCanisterUpgradeArgs {
                version: BuildVersion::new(0, 0, 2), // init will set this to "0.0.0" in test setup
                commit_hash: "zyxwvt".to_string(),
            })
        ).unwrap();
        pic.upgrade_canister(
            gldt_swap,
            gldt_swap_canister_wasm,
            gldt_swap_init_args,
            Some(controller)
        ).unwrap();
        tick_n_blocks(pic, 5);
        let version = get_version(
            pic,
            Principal::anonymous(),
            archive_canister.canister_id.clone(),
            &()
        );
        assert_eq!(version, BuildVersion::new(0, 0, 2));

        // check controllers match
    }
}
