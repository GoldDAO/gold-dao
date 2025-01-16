use candid::Nat;

use self::setup::{GldtStakeTestEnv, GldtStakeTestEnvBuilder};

pub mod setup;
pub mod setup_gldt_stake;
pub mod setup_ledger;
pub mod setup_rewards;
pub mod setup_sns;

pub fn default_test_setup() -> GldtStakeTestEnv {
    GldtStakeTestEnvBuilder::new()
        .add_token_ledger("ICP", &mut vec![], Nat::from(10_000u64))
        .add_token_ledger("OGY", &mut vec![], Nat::from(200_000u64))
        .add_token_ledger("GLDGov", &mut vec![], Nat::from(100_000u64))
        .add_token_ledger("GLDT", &mut vec![], Nat::from(1_000_000u64))
        .build()
}
