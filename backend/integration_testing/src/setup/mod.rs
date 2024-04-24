use candid::Principal;

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
        .add_users(users)
        .build()
}

pub fn test_setup_with_no_neuron_hotkeys() -> RewardsTestEnv {
    RewardsTestEnvBuilder::new()
        .add_random_neurons(10)
        .add_token_ledger("ICP", &mut vec![])
        .add_token_ledger("OGY", &mut vec![])
        .add_token_ledger("GLDGov", &mut vec![])
        .add_users(vec![])
        .build()
}
