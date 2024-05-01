use candid::{ Nat, Principal };

use self::setup::{ RewardsTestEnv, RewardsTestEnvBuilder };

pub mod setup;
pub mod setup_ledger;
pub mod setup_sns;
pub mod setup_rewards;

pub fn default_test_setup() -> RewardsTestEnv {
    let users = vec![
        Principal::from_slice(&[0, 0, 0, 1, 0, 1, 0, 1, 0, 1]),
        Principal::from_slice(&[0, 0, 0, 1, 0, 2, 0, 2, 0, 2])
    ];

    RewardsTestEnvBuilder::new()
        .add_random_neurons(10)
        .add_token_ledger("ICP", &mut vec![])
        .add_token_ledger("OGY", &mut vec![])
        .add_token_ledger("GLDGov", &mut vec![])
        .with_reward_pools(Nat::from(100_000_000_000u64))
        .add_users(users)
        .build()
}

pub fn test_setup_with_no_neuron_hotkeys() -> RewardsTestEnv {
    RewardsTestEnvBuilder::new()
        .add_random_neurons(10)
        .add_token_ledger("ICP", &mut vec![])
        .add_token_ledger("OGY", &mut vec![])
        .add_token_ledger("GLDGov", &mut vec![])
        .with_reward_pools(Nat::from(100_000_000_000u64))
        .add_users(vec![])
        .build()
}

// minting account is always setup with 1_000_000_000_000_000 and this test env keeps it consistent by not minting to any reward pools as we do in other setups
pub fn test_setup_with_no_reward_pool_mint() -> RewardsTestEnv {
    let users = vec![
        Principal::from_slice(&[0, 0, 0, 1, 0, 1, 0, 1, 0, 1]),
        Principal::from_slice(&[0, 0, 0, 1, 0, 2, 0, 2, 0, 2])
    ];

    RewardsTestEnvBuilder::new()
        .add_random_neurons(10)
        .add_token_ledger("ICP", &mut vec![])
        .add_token_ledger("OGY", &mut vec![])
        .add_token_ledger("GLDGov", &mut vec![])
        .add_users(users)
        .build()
}
