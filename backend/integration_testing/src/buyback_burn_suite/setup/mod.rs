use candid::Nat;

use self::setup::{TestEnv, TestEnvBuilder};

pub mod setup;
pub mod setup_buyback_burn;

pub fn default_test_setup() -> TestEnv {
    TestEnvBuilder::new().build()
}
