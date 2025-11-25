use self::setup::{TestEnv, TestEnvBuilder};
pub mod setup;
pub mod setup_gldt_ledger;
pub mod setup_gldt_swap;
pub mod setup_index;

pub fn default_test_setup() -> TestEnv {
    let mut test_env = TestEnvBuilder::new();

    test_env.build()
}
