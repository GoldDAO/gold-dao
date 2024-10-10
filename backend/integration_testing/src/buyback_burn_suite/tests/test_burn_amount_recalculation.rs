use crate::client::buyback_burn::*;
use std::time::Duration;

use crate::{ buyback_burn_suite::setup::default_test_setup, utils::tick_n_blocks };

#[test]
fn test_process_neurons_happy_path() {
    let mut test_env = default_test_setup();
    let buyback_burn_id = test_env.buyback_burn_id;

    tick_n_blocks(&test_env.pic, 100);
    test_env.pic.advance_time(Duration::from_secs(14400));
    tick_n_blocks(&test_env.pic, 100);

    let result0 = get_last_burn_amount_update(
        &mut test_env.pic,
        test_env.controller,
        buyback_burn_id,
        &()
    );
    println!("result0 {:?}", result0);

    tick_n_blocks(&test_env.pic, 100);
    test_env.pic.advance_time(Duration::from_secs(14400));
    tick_n_blocks(&test_env.pic, 100);

    let result1 = get_last_burn_amount_update(
        &mut test_env.pic,
        test_env.controller,
        buyback_burn_id,
        &()
    );
    println!("result1 {:?}", result1);

    tick_n_blocks(&test_env.pic, 100);
    test_env.pic.advance_time(Duration::from_secs(604801));
    tick_n_blocks(&test_env.pic, 100);

    let result2 = get_last_burn_amount_update(
        &mut test_env.pic,
        test_env.controller,
        buyback_burn_id,
        &()
    );
    println!("result2 {:?}", result2); // Thu May 13 2021 19:33:51 GMT+0000

    tick_n_blocks(&test_env.pic, 100);
    test_env.pic.advance_time(Duration::from_secs(604801));
    tick_n_blocks(&test_env.pic, 100);

    let result3 = get_last_burn_amount_update(
        &mut test_env.pic,
        test_env.controller,
        buyback_burn_id,
        &()
    );
    println!("result3 {:?}", result3); // Thu May 20 2021 19:33:52 GMT+0000

    assert!(result0 == result1);
    assert!(result2 > result1);
    assert!(result3 > result2);
}
