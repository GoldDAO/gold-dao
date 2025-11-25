use self::setup_one_canister::{TestEnv, TestEnvBuilder};
use crate::gldt_swap_suite::setup::setup_real_gold_config::RealDataTestEnvBuilder;
pub mod setup_gldt_ledger;
pub mod setup_gldt_swap;
use crate::gldt_swap_suite::setup::setup_real_gold_config::RealDataTestEnv;
pub mod setup_one_canister;
pub mod setup_real_gold_config;

pub fn default_test_setup() -> TestEnv {
    let mut test_env = TestEnvBuilder::new();

    test_env.build()
}

pub fn real_data_test_setup() -> RealDataTestEnv {
    let mut test_env = RealDataTestEnvBuilder::new();

    test_env.build()
}
