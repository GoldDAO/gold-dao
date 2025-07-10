use crate::{buyback_burn_suite::setup::default_test_setup, utils::tick_n_blocks};
use std::time::Duration;

#[test]
fn test_run_job_at() {
    let test_env = default_test_setup();

    test_env
        .pic
        .advance_time(Duration::from_secs(3 * 24 * 60 * 60));
    tick_n_blocks(&test_env.pic, 100);
    println!("tick_n_blocks");
}
