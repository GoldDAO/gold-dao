use std::time::Duration;

use crate::client::gldt_swap::{get_swap, insert_fake_bulk_swaps, insert_fake_swap};
use crate::gldt_swap_suite::{init, CanisterIds, PrincipalIds, TestEnv};
use crate::utils::tick_n_blocks;

use candid::{Nat, Principal};
use canister_time::{timestamp_millis, HOUR_IN_MS, MINUTE_IN_MS, WEEK_IN_MS};
use gldt_swap_common::gldt::GldtNumTokens;
use gldt_swap_common::nft::NftID;
use gldt_swap_common::swap::{
    SwapDetailForward, SwapErrorForward, SwapIndex, SwapInfo, SwapStatus, SwapStatusForward,
};
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::PocketIc;

fn insert_bulk_fake_swaps(
    pic: &mut PocketIc,
    start_index: usize,
    end_index: usize,
    controller: Principal,
    gldt_swap: Principal,
) -> (Principal, Principal) {
    let user_a = Principal::from_slice(&[
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
    ]);
    let user_b = Principal::from_slice(&[
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8,
    ]);
    let mut swaps: Vec<SwapInfo> = vec![];
    for i in start_index..end_index {
        let user = if i == 0 {
            user_a
        } else if i % 2 == 0 {
            user_b
        } else {
            user_a
        };
        let time_now = timestamp_millis() + (Duration::from_millis(WEEK_IN_MS).as_millis() as u64);
        swaps.push(SwapInfo::Forward(SwapDetailForward {
            index: SwapIndex::from(i),
            nft_id: NftID(Nat::from(i)),
            nft_id_string: i.to_string(),
            nft_canister: Principal::anonymous(),
            status: SwapStatusForward::Failed(SwapErrorForward::Expired(Box::new(
                SwapStatusForward::BidRequest,
            ))),
            sale_id: String::from(""),
            created_at: time_now,
            tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
            escrow_sub_account: [0u8; 32],
            gldt_receiver: Account {
                owner: user,
                subaccount: None,
            },
        }));
    }
    insert_fake_bulk_swaps(pic, controller.clone(), gldt_swap.clone(), &swaps).unwrap();
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
    tick_n_blocks(pic, 10);
    pic.advance_time(Duration::from_millis(MINUTE_IN_MS));
    tick_n_blocks(pic, 10);
    (user_a, user_b)
}
#[cfg(test)]
mod tests {
    use candid::Encode;
    use gldt_swap_common::{archive::ArchiveCanister, nft::NftCanisterConf, swap::SwapId};

    use gldt_swap_api_archive::get_archive_swaps::{
        Args as GetArchiveSwapArgs, Response as GetArchiveSwapResponse,
    };
    use gldt_swap_api_canister::{
        get_historic_swaps::{Args as GetHistoricSwapsArgs, GetHistoricSwapsError},
        get_historic_swaps_by_user::Args as GetHistoricSwapsByUserArgs,
        init::InitArgs as GldtSwapCanisterInitArgs,
        lifecycle::Args as GldtSwapCanisterArgs,
        post_upgrade::UpgradeArgs as GldtSwapCanisterUpgradeArgs,
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
            get_archive_canisters, get_archive_swaps, get_historic_swaps,
            get_historic_swaps_by_user, get_history_total, get_version,
        },
        wasms,
    };

    use super::*;
    #[test]
    pub fn archive_features_work_correctly() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_swap,
                    gldt_ledger,
                    ogy_ledger,
                    ..
                },
            principal_ids: PrincipalIds { controller, .. },
        } = env;
        tick_n_blocks(pic, 2); // need to wait for cron job to finish creating the archive
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 1);

        // In test mode there is a threshhold of approximately 10MB before a new archive canister is created.
        // at 366 a new archive is created ( based on memory size ).the buffer ( 100 ) means that no new swaps will be stored in archive 2 466
        // test that the correct index
        let (user_a, _) = insert_bulk_fake_swaps(pic, 0, 366, controller, gldt_swap);

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        println!("///////////////{archive_canisters:?}");
        assert_eq!(archive_canisters.len(), 2);
        let ArchiveCanister {
            start_index: second_archive_start_index,
            ..
        } = archive_canisters.last().unwrap();
        println!("archive canister 2 index : {second_archive_start_index:?}");
        println!("archive canisters : {archive_canisters:?}");
        let total_swaps: Nat = get_history_total(pic, Principal::anonymous(), gldt_swap, &None);
        assert_eq!(total_swaps, Nat::from(366u64));
        let mut start_swap_index = 0usize;

        // test all individual swaps are locatable first with no duplicates or extra swaps
        // for i in 0..366u64 {
        //     let res = get_swap(
        //         pic,
        //         Principal::anonymous(),
        //         gldt_swap,
        //         &SwapId(NftID(Nat::from(i)), Nat::from(i))
        //     ).unwrap();
        //     assert_eq!(res.0.1, Nat::from(i));
        // }
        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(365u64)), Nat::from(365u64)),
        );
        assert_eq!(res.is_some(), true);
        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(366u64)), Nat::from(366u64)),
        );
        assert_eq!(res.is_none(), true);

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());

        // there should be 2 archive canisters
        assert_eq!(archive_canisters.len(), 2);

        let archive_canister_1 = archive_canisters[0].clone();
        let archive_canister_2 = archive_canisters[1].clone();

        println!("{archive_canister_1:?}");
        println!("{archive_canister_2:?}");

        assert_eq!(archive_canister_1.start_index, Nat::from(0u64));
        assert_eq!(archive_canister_1.end_index, Some(Nat::from(465u64)));

        assert_eq!(archive_canister_2.start_index, Nat::from(466u64));
        assert_eq!(archive_canister_2.end_index, None);

        // we're in the buffer zone where archive_1 still has an extra 100 swaps to fill up until 466 is reached
        // archive 1 should have 0 - 365
        // archive 2 should have 0

        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_2.canister_id,
            &(GetArchiveSwapArgs {
                start: Nat::from(466u64),
                limit: 200,
                user_principal: None,
            }),
        );
        assert_eq!(res.len(), 0);
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_1.canister_id,
            &(GetArchiveSwapArgs {
                start: Nat::from(465u64),
                limit: 200,
                user_principal: None,
            }),
        );
        assert_eq!(res.len(), 200);
        match res.first() {
            Some(swap) => {
                assert_eq!(swap.0 .1, Nat::from(365u64));
            }
            None => {
                panic!("swap not found");
            }
        }

        // now we're going to fill up archive 1 and test that swaps are automatically inserted into archive 2
        // archive 1 is currently at 365 index out of a max 465 so we're going to insert 110 swaps
        // 100 to fill up archive 1 and 10 to check in archive 2
        let (user_a, _) = insert_bulk_fake_swaps(pic, 366, 476, controller, gldt_swap);

        // recheck the no new archives got created
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 2);

        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_1.canister_id,
            &(GetArchiveSwapArgs {
                start: Nat::from(465u64),
                limit: 50,
                user_principal: None,
            }),
        );
        assert_eq!(res.len(), 50);
        match res.first() {
            Some(swap) => {
                assert_eq!(swap.0 .1, Nat::from(465u64));
            }
            None => {
                panic!("swap not found");
            }
        }
        let res = get_archive_swaps(
            pic,
            Principal::anonymous(),
            archive_canister_2.canister_id,
            &(GetArchiveSwapArgs {
                start: Nat::from(476u64),
                limit: 200,
                user_principal: None,
            }),
        );
        assert_eq!(res.len(), 10);
    }

    #[test]
    pub fn get_historic_swaps_works_correctly() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_swap,
                    gldt_ledger,
                    ogy_ledger,
                    ..
                },
            principal_ids: PrincipalIds { controller, .. },
        } = env;
        tick_n_blocks(pic, 2); // need to wait for cron job to finish creating the archive
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 1);

        let (user_a, _) = insert_bulk_fake_swaps(pic, 0, 366, controller, gldt_swap);
        let (user_a, _) = insert_bulk_fake_swaps(pic, 366, 476, controller, gldt_swap);

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 2);
        let ArchiveCanister {
            start_index: second_archive_start_index,
            ..
        } = archive_canisters.last().unwrap();
        println!("archive canister 2 index : {second_archive_start_index:?}");
        println!("archive canisters : {archive_canisters:?}");
        let total_swaps: Nat = get_history_total(pic, Principal::anonymous(), gldt_swap, &None);
        assert_eq!(total_swaps, Nat::from(476u64));
        let mut page = 0usize;

        // test all individual swaps are locatable first with no duplicates or extra swaps
        // for i in 0..476u64 {
        //     let res = get_swap(
        //         pic,
        //         Principal::anonymous(),
        //         gldt_swap,
        //         &SwapId(NftID(Nat::from(i)), Nat::from(i))
        //     ).unwrap();
        //     assert_eq!(res.0.1, Nat::from(i));
        // }
        let res = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(476u64)), Nat::from(476u64)),
        );
        assert_eq!(res.is_none(), true);

        // the starting index should be retrievable
        let swap = get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(
                NftID(second_archive_start_index.clone()),
                Nat::from(second_archive_start_index.clone()),
            ),
        );
        assert_eq!(swap.is_some(), true);

        // test simple pagination - should be able to go all the way from 249 to 0
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: page,
                limit: 100,
            }),
        )
        .unwrap();
        assert_eq!(res.len(), 100);

        for (swap, expected_id) in res
            .iter()
            .zip((0..100).rev().map(|i| Nat::from(i + 376u64)))
        {
            assert_eq!(expected_id, swap.0 .1);
        }

        page += 1;
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: page,
                limit: 100,
            }),
        )
        .unwrap();
        assert_eq!(res.len(), 100);
        for (swap, expected_id) in res
            .iter()
            .zip((0..100).rev().map(|i| Nat::from(i + 276u64)))
        {
            assert_eq!(expected_id, swap.0 .1);
        }

        page += 1;
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: page,
                limit: 100,
            }),
        )
        .unwrap();
        assert_eq!(res.len(), 100);
        for (swap, expected_id) in res
            .iter()
            .zip((0..100).rev().map(|i| Nat::from(i + 176u64)))
        {
            assert_eq!(expected_id, swap.0 .1);
        }

        page += 1;
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: page,
                limit: 100,
            }),
        )
        .unwrap();
        assert_eq!(res.len(), 100);
        for (swap, expected_id) in res.iter().zip((0..100).rev().map(|i| Nat::from(i + 76u64))) {
            assert_eq!(expected_id, swap.0 .1);
        }

        page += 1;
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: page,
                limit: 100,
            }),
        )
        .unwrap();
        assert_eq!(res.len(), 76);
        for (swap, expected_id) in res.iter().zip((0..76u64).rev()) {
            assert_eq!(Nat::from(expected_id), swap.0 .1);
        }

        // test a limit that is too large
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: page,
                limit: 201,
            }),
        );
        matches!(res, Err(GetHistoricSwapsError::LimitTooLarge(_)));

        // // test when start + limit is more than the total number of swaps

        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: 2, // 300
                limit: 200,
            }),
        )
        .unwrap();
        assert_eq!(res.len(), 76); // 476 total swaps - ( 200 * 2 )

        // test a page that should return no swaps
        let res = get_historic_swaps(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsArgs {
                page: 3, // start at 400
                limit: 200,
            }),
        )
        .unwrap();
        if &res.len() > &0usize {
            let last_swap = res.get(0);
            println!("{last_swap:?}");
        }
        assert_eq!(res.len(), 0);

        // test getting user historic swaps
        let mut running_user_total = 0u64;
        let res = get_historic_swaps_by_user(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsByUserArgs {
                page: 0,
                limit: 100,
                user: user_a,
            }),
        )
        .unwrap();
        assert_eq!(res.len(), 100);
        running_user_total += 100;
        assert_eq!(res.first().unwrap().0 .1, Nat::from(475u64));
        assert_eq!(res.last().unwrap().0 .1, Nat::from(277u64));

        let res = get_historic_swaps_by_user(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsByUserArgs {
                page: 1,
                limit: 100,
                user: user_a,
            }),
        )
        .unwrap();
        assert_eq!(res.len(), 100);
        running_user_total += 100;
        assert_eq!(res.first().unwrap().0 .1, Nat::from(275u64));
        assert_eq!(res.last().unwrap().0 .1, Nat::from(77u64));

        let res = get_historic_swaps_by_user(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &(GetHistoricSwapsByUserArgs {
                page: 2,
                limit: 100,
                user: user_a,
            }),
        )
        .unwrap();
        assert_eq!(res.len(), 39);
        running_user_total += 39;
        assert_eq!(res.first().unwrap().0 .1, Nat::from(75u64));
        assert_eq!(res.last().unwrap().0 .1, Nat::from(0u64));

        // check user_history_total works
        let user_history_total =
            get_history_total(pic, Principal::anonymous(), gldt_swap, &Some(user_a));
        assert_eq!(user_history_total, Nat::from(running_user_total));

        // will upgrade multiple canisters
        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        for archive in archive_canisters {
            let version = get_version(
                pic,
                Principal::anonymous(),
                archive.canister_id.clone(),
                &(),
            );
            assert_eq!(version, BuildVersion::new(0, 0, 0));
        }

        // upgrading should work fine
        let gldt_swap_canister_wasm: Vec<u8> = wasms::GLDT_SWAP.clone();
        let gldt_swap_init_args = Encode!(&GldtSwapCanisterArgs::Upgrade(
            GldtSwapCanisterUpgradeArgs {
                version: BuildVersion::new(0, 0, 2),
                commit_hash: "zyxwvut".to_string(),
            }
        ))
        .unwrap();
        pic.upgrade_canister(
            gldt_swap,
            gldt_swap_canister_wasm,
            gldt_swap_init_args,
            Some(controller),
        )
        .unwrap();
        tick_n_blocks(pic, 20);

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        for archive in archive_canisters {
            let version = get_version(
                pic,
                Principal::anonymous(),
                archive.canister_id.clone(),
                &(),
            );
            assert_eq!(version, BuildVersion::new(0, 0, 2));
        }

        // get a swap from both archive canisters
        get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(50u64)), Nat::from(50u64)),
        )
        .unwrap();

        get_swap(
            pic,
            Principal::anonymous(),
            gldt_swap,
            &SwapId(NftID(Nat::from(210u64)), Nat::from(210u64)),
        )
        .unwrap();

        let archive_canisters = get_archive_canisters(pic, Principal::anonymous(), gldt_swap, &());
        assert_eq!(archive_canisters.len(), 2);
    }

    #[test]
    fn archive_will_upgrade() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids:
                CanisterIds {
                    origyn_nft,
                    gldt_swap,
                    gldt_ledger,
                    ogy_ledger,
                    ..
                },
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
            &(),
        );
        assert_eq!(version, BuildVersion::new(0, 0, 0));

        // upgrading should work fine
        let gldt_swap_canister_wasm: Vec<u8> = wasms::GLDT_SWAP.clone();
        let gldt_swap_init_args = Encode!(&GldtSwapCanisterArgs::Upgrade(
            GldtSwapCanisterUpgradeArgs {
                version: BuildVersion::new(0, 0, 2), // init will set this to "0.0.0" in test setup
                commit_hash: "zyxwvt".to_string(),
            }
        ))
        .unwrap();
        pic.upgrade_canister(
            gldt_swap,
            gldt_swap_canister_wasm,
            gldt_swap_init_args,
            Some(controller),
        )
        .unwrap();
        tick_n_blocks(pic, 5);
        let version = get_version(
            pic,
            Principal::anonymous(),
            archive_canister.canister_id.clone(),
            &(),
        );
        assert_eq!(version, BuildVersion::new(0, 0, 2));

        // check controllers match
    }
}
