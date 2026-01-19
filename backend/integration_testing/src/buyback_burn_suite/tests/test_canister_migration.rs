use crate::buyback_burn_suite::setup::setup_buyback_burn::upgrade_buyback_burn_canister;
use crate::buyback_burn_suite::setup::setup_buyback_burn_old::setup_old_buyback_burn_canister;
use crate::{buyback_burn_suite::setup::default_test_setup, utils::tick_n_blocks};

#[test]
fn test_migration_happy_path() {
    let test_env = default_test_setup();
    let pic = test_env.pic;
    let buyback_burn_canister_id = test_env.buyback_burn_id;
    setup_old_buyback_burn_canister(&pic, buyback_burn_canister_id, &test_env.controller);
    tick_n_blocks(&pic, 5);

    let status = pic.canister_status(buyback_burn_canister_id, Some(test_env.controller));
    println!("Canister status before migration: {:?}", status);

    upgrade_buyback_burn_canister(&pic, buyback_burn_canister_id, &test_env.controller).unwrap();
    tick_n_blocks(&pic, 20);

    let status = pic.canister_status(buyback_burn_canister_id, Some(test_env.controller));
    println!("Canister status after migration: {:?}", status);
}
