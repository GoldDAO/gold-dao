use candid::{Encode, Nat, Principal};
use canister_time::{DAY_IN_MS, HOUR_IN_MS, SECOND_IN_MS};
use gldt_stake_api_canister::claim_reward;
use gldt_stake_common::stake_position::{ClaimRewardErrors, StakePositionId};
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;
use types::BuildVersion;

use crate::client::gldt_archive::get_version;
use crate::client::gldt_stake::{
    claim_reward, get_active_user_positions, get_archive_canisters, get_historic_positions_by_user,
    get_historic_positions_total_by_user,
};
use crate::client::pocket::unwrap_response;
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::gldt_stake_suite::utils::{
    add_rewards_to_neurons, create_multiple_early_unstaked_positions, create_stake_position_util,
};
use crate::wasms;
use crate::{
    client::icrc1::client::balance_of, gldt_stake_suite::setup::default_test_setup,
    utils::tick_n_blocks,
};
use gldt_stake_api_canister::get_historic_positions_by_user::Args as GetHistoricPositionsByUserArgs;
use gldt_stake_api_canister::lifecycle::Args as GldtStakeCanisterArgs;
use gldt_stake_api_canister::post_upgrade::UpgradeArgs as GldtStakeUpgradeArgs;

#[test]
fn creates_new_archive_on_init() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        gld_rewards_canister_id,
        neuron_data,
        ledger_fees,
        ..
    } = test_env;

    let pic_borrowed = &pic.borrow();

    pic_borrowed.advance_time(Duration::from_millis(SECOND_IN_MS));
    tick_n_blocks(pic_borrowed, 10);
    let res = get_archive_canisters(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &(),
    );
    assert_eq!(res.len(), 1);
}

#[test]
fn test_get_historic_user_positions() {
    let mut test_env = default_test_setup();
    // println!("test_env : {:?}", test_env);

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        gld_rewards_canister_id,
        neuron_data,
        ledger_fees,
        ..
    } = test_env;

    let pic_borrowed = &pic.borrow();

    println!("gldt_stake_canister_id : {gldt_stake_canister_id}");
    println!("controller : {controller}");
    token_ledgers.iter().for_each(|(name, principal)| {
        println!("{name} : {principal}");
    });
    println!("gld_rewards_canister_id : {gld_rewards_canister_id:?}");

    let gldt_ledger_id = token_ledgers.get("gldt_ledger_canister_id").unwrap();

    const POSITIONS_PER_USER: usize = 70;
    let positions_by_user = create_multiple_early_unstaked_positions(
        pic_borrowed,
        controller,
        &token_ledgers,
        gldt_stake_canister_id,
        2,
        POSITIONS_PER_USER,
    );
    tick_n_blocks(pic_borrowed, 5);

    assert_eq!(positions_by_user.len(), 2);

    let user_1 = positions_by_user.get(0).unwrap().0.clone();
    println!("{user_1}");
    let user_2 = positions_by_user.get(1).unwrap().0.clone();
    println!("{user_2}");

    let user_1_positions = positions_by_user.get(0).unwrap().1.clone();
    let user_2_positions = positions_by_user.get(1).unwrap().1.clone();

    assert_eq!(user_1_positions.len(), POSITIONS_PER_USER);
    assert_eq!(user_2_positions.len(), POSITIONS_PER_USER);

    // based on this setup of 140 total swaps and the artificially inflated size of StakePosition for the inttest flag
    // we expect a new archive split at around 113 added stake positions to fill up 16mb
    // we expect a total of 2 archive canisters
    // we expect that user_1 has all their positions in the first archive
    // we expect that user_2 has their positions split between the first and second archive

    // -----------------------------------
    // --------- begin tests -------------
    // -----------------------------------

    // basic test
    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_2,
            start: 0,
            limit: 100,
        },
    );

    assert_eq!(res.len(), POSITIONS_PER_USER);
    res.iter().for_each(|a| {
        println!("id : {} - prin :{} ", a.id, a.owned_by);
    });
    assert_eq!(res.get(0).unwrap().id, 70);
    assert_eq!(res.last().unwrap().id, 139);

    // pagination by 10 each time using user 1 ( id range 0 - 69 )
    let mut start = 0;
    let limit = 10;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_1,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 0);
    assert_eq!(res.last().unwrap().id, 9);
    start = 10;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_1,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 10);
    assert_eq!(res.last().unwrap().id, 19);
    start = 20;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_1,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 20);
    assert_eq!(res.last().unwrap().id, 29);
    start = 30;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_1,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 30);
    assert_eq!(res.last().unwrap().id, 39);
    start = 40;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_1,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 40);
    assert_eq!(res.last().unwrap().id, 49);
    start = 50;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_1,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 50);
    assert_eq!(res.last().unwrap().id, 59);
    start = 60;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_1,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 60);
    assert_eq!(res.last().unwrap().id, 69);
    start = 70;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_1,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 0);

    // repeat test for above using user_2 - (id range 70 - 139)
    let mut start = 0;
    let limit = 10;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_2,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 70);
    assert_eq!(res.last().unwrap().id, 79);
    start = 10;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_2,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 80);
    assert_eq!(res.last().unwrap().id, 89);
    start = 20;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_2,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 90);
    assert_eq!(res.last().unwrap().id, 99);
    start = 30;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_2,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 100);
    assert_eq!(res.last().unwrap().id, 109);
    start = 40;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_2,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 110);
    assert_eq!(res.last().unwrap().id, 119);
    start = 50;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_2,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 120);
    assert_eq!(res.last().unwrap().id, 129);
    start = 60;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_2,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 10);
    assert_eq!(res.get(0).unwrap().id, 130);
    assert_eq!(res.last().unwrap().id, 139);
    start = 70;

    let res = get_historic_positions_by_user(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &GetHistoricPositionsByUserArgs {
            user: user_2,
            start: start,
            limit: limit,
        },
    );
    assert_eq!(res.len(), 0);

    // test getting the totals for each user
    let total_number_of_positions =
        get_historic_positions_total_by_user(pic_borrowed, user_1, gldt_stake_canister_id, &(None));
    assert_eq!(total_number_of_positions, POSITIONS_PER_USER);

    let total_number_of_positions =
        get_historic_positions_total_by_user(pic_borrowed, user_2, gldt_stake_canister_id, &(None));
    assert_eq!(total_number_of_positions, POSITIONS_PER_USER);

    // test archive state - the second archvie canister should be active and the first should be false because it is full

    let archive_canisters = get_archive_canisters(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &(),
    );

    assert_eq!(archive_canisters.get(0).unwrap().active, false);
    assert_eq!(archive_canisters.get(1).unwrap().active, true);
}

#[test]
fn test_upgrading_will_also_upgrade_archives() {
    let mut test_env = default_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        token_ledgers,
        gldt_stake_canister_id,
        gld_rewards_canister_id,
        neuron_data,
        ledger_fees,
        ..
    } = test_env;
    let pic_borrowed = &pic.borrow();

    tick_n_blocks(pic_borrowed, 1);

    let archive_canisters = get_archive_canisters(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &(),
    );
    for archive in archive_canisters {
        let version = get_version(
            pic_borrowed,
            Principal::anonymous(),
            archive.canister_id.clone(),
            &(),
        );
        assert_eq!(version, BuildVersion::new(0, 0, 0));
    }
    tick_n_blocks(pic_borrowed, 20);
    // upgrading should work fine
    let gldt_stake_canister_wasm: Vec<u8> = wasms::GLDT_STAKE.clone();
    let gldt_stake_init_args = Encode!(&GldtStakeCanisterArgs::Upgrade(GldtStakeUpgradeArgs {
        version: BuildVersion::new(0, 0, 2),
        commit_hash: "zyxwvut".to_string(),
    }))
    .unwrap();

    pic_borrowed
        .upgrade_canister(
            gldt_stake_canister_id,
            gldt_stake_canister_wasm,
            gldt_stake_init_args,
            Some(controller),
        )
        .unwrap();
    tick_n_blocks(pic_borrowed, 5);
    let archive_canisters = get_archive_canisters(
        pic_borrowed,
        Principal::anonymous(),
        gldt_stake_canister_id,
        &(),
    );
    for archive in archive_canisters {
        let version = get_version(
            pic_borrowed,
            Principal::anonymous(),
            archive.canister_id.clone(),
            &(),
        );
        assert_eq!(version, BuildVersion::new(0, 0, 2));
    }
}
