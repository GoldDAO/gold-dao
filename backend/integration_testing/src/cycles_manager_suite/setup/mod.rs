use candid::{Nat, Principal};

use self::setup::{CyclesManagerEnv, CyclesManagerTestEnvBuilder};

pub mod setup;
pub mod setup_cycles_manager;
pub mod setup_ledger;
pub mod setup_rewards;
pub mod setup_sns;

pub fn default_test_setup() -> CyclesManagerEnv {
    let users = vec![
        Principal::from_slice(&[0, 0, 0, 1, 0, 1, 0, 1, 0, 1]),
        Principal::from_slice(&[0, 0, 0, 1, 0, 2, 0, 2, 0, 2]),
    ];

    CyclesManagerTestEnvBuilder::new()
        .add_random_neurons(10)
        .add_token_ledger("ICP", &mut vec![], Nat::from(10_000u64))
        .add_token_ledger("OGY", &mut vec![], Nat::from(200_000u64))
        .add_token_ledger("GLDGov", &mut vec![], Nat::from(100_000u64))
        .with_reward_pools(Nat::from(100_000_000_000u64))
        .add_users(users)
        .build()
}

pub fn test_setup_with_no_neuron_hotkeys() -> CyclesManagerEnv {
    CyclesManagerTestEnvBuilder::new()
        .add_random_neurons(10)
        .add_token_ledger("ICP", &mut vec![], Nat::from(10_000u64))
        .add_token_ledger("OGY", &mut vec![], Nat::from(200_000u64))
        .add_token_ledger("GLDGov", &mut vec![], Nat::from(100_000u64))
        .with_reward_pools(Nat::from(100_000_000_000u64))
        .add_users(vec![])
        .build()
}

// minting account is always setup with 1_000_000_000_000_000 and this test env keeps it consistent by not minting to any reward pools as we do in other setups
pub fn test_setup_with_no_reward_pool_mint() -> CyclesManagerEnv {
    let users = vec![
        Principal::from_slice(&[0, 0, 0, 1, 0, 1, 0, 1, 0, 1]),
        Principal::from_slice(&[0, 0, 0, 1, 0, 2, 0, 2, 0, 2]),
    ];

    CyclesManagerTestEnvBuilder::new()
        .add_random_neurons(10)
        .add_token_ledger("ICP", &mut vec![], Nat::from(10_000u64))
        .add_token_ledger("OGY", &mut vec![], Nat::from(200_000u64))
        .add_token_ledger("GLDGov", &mut vec![], Nat::from(100_000u64))
        .add_users(users)
        .build()
}
