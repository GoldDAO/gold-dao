use self::setup::{TestEnv, TestEnvBuilder};

pub mod setup;
pub mod setup_buyback_burn;
pub mod setup_buyback_burn_old;

pub fn default_test_setup() -> TestEnv {
    TestEnvBuilder::new().build()
}
