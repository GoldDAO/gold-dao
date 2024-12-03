use candid::{CandidType, Deserialize};
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use std::time::Duration;

use crate::{buyback_burn_suite::setup::default_test_setup, utils::tick_n_blocks};

#[test]
fn test_run_job_at() {
    let mut test_env = default_test_setup();

    test_env
        .pic
        .advance_time(Duration::from_secs(3 * 24 * 60 * 60));
    tick_n_blocks(&test_env.pic, 100);
    println!("tick_n_blocks");
}
