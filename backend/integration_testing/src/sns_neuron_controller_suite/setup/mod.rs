use candid::Nat;

use self::setup::{SNCTestEnv, SNCTestEnvBuilder};

pub mod setup;
pub mod setup_ledger;
pub mod setup_rewards;
pub mod setup_sns;
pub mod setup_sns_neuron_controller;

pub fn default_test_setup() -> SNCTestEnv {
    SNCTestEnvBuilder::new()
        .add_token_ledger("ICP", &mut vec![], Nat::from(10_000u64))
        .add_token_ledger("OGY", &mut vec![], Nat::from(200_000u64))
        .add_token_ledger("GLDGov", &mut vec![], Nat::from(100_000u64))
        .build()
}
