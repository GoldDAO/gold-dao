// Import the necessary modules and types
use crate::client::cycles_manager;
use crate::cycles_manager_suite::setup::default_test_setup;
use crate::utils::tick_n_blocks;
use std::time::Duration;
use tracing_subscriber::FmtSubscriber;

// Define the test function
#[test]
fn test_cycles_management() {
    let mut test_env = default_test_setup();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::ERROR)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Get canisters ID:
    let cycles_manager_id = test_env.cycles_manager_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    // Get cycles_manager balance
    let initial_cycles_manager_balance = test_env.pic.cycle_balance(cycles_manager_id);
    println!(
        "initial_cycles_manager_balance: {}",
        initial_cycles_manager_balance
    );

    // Get rewards_canister balance (initially it's greater than the threshold)
    let initial_rewards_canister_balance = test_env.pic.cycle_balance(rewards_canister_id);
    println!(
        "initial_rewards_canister_balance: {}",
        initial_rewards_canister_balance
    );

    cycles_manager::update_config(
        &mut test_env.pic,
        test_env.controller,
        cycles_manager_id,
        &cycles_manager_canister::update_config::Args {
            min_cycles_balance: Some(200_000_000_000_000),
            min_interval: None,
            max_top_up_amount: None,
        },
    );

    // Simulate the time passing ()
    test_env.pic.advance_time(Duration::from_secs(36000000));
    tick_n_blocks(&test_env.pic, 100);

    let current_cycles_manager_balance = test_env.pic.cycle_balance(cycles_manager_id);

    // Check if the rewards canister has low balance
    let current_rewards_canister_balance = test_env.pic.cycle_balance(rewards_canister_id);

    // The threshold is set up to 200_000_000_000_000
    assert!(current_rewards_canister_balance > 200_000_000_000_000);
}
