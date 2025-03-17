use candid::Principal;
use canister_time::SECOND_IN_MS;
use gldt_stake_common::archive::MANAGE_ARCHIVE_CYCLE_INTERVAL;
use std::time::Duration;

use crate::client::gldt_stake::get_archive_canisters;
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;

use crate::{gldt_stake_suite::setup::default_test_setup, utils::tick_n_blocks};

#[test]
fn cycles_are_transfered_to_archive_correctly() {
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

    pic.advance_time(Duration::from_millis(SECOND_IN_MS));
    tick_n_blocks(pic, 10);
    let res = get_archive_canisters(pic, Principal::anonymous(), gldt_stake_canister_id, &());
    let archive_canister_id = res.get(0).unwrap().clone().canister_id;
    assert_eq!(res.len(), 1);

    let archive_canister_cycles = pic.cycle_balance(archive_canister_id);
    println!("cycles before : {archive_canister_cycles}");

    assert!(archive_canister_cycles < 5_000_000_000_000 as u128);
    pic.advance_time(Duration::from_millis(MANAGE_ARCHIVE_CYCLE_INTERVAL));
    tick_n_blocks(pic, 2);
    pic.advance_time(Duration::from_millis(MANAGE_ARCHIVE_CYCLE_INTERVAL));
    tick_n_blocks(pic, 2);

    let archive_canister_cycles = pic.cycle_balance(archive_canister_id);
    println!("cycles after : {archive_canister_cycles}");
    assert!(archive_canister_cycles > 5_000_000_000_000 as u128);
}
