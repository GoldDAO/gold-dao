// Import the necessary modules and types
use crate::cycles_manager_suite::setup::default_test_setup;
use crate::utils::tick_n_blocks;
use std::time::Duration;
use types::Cycles;
const T: Cycles = 1_000_000_000_000;

// Define the test function
#[test]
fn test_cycles_management() {
    let mut test_env = default_test_setup();

    // Get canisters ID:
    let cycles_manager_id = test_env.cycles_manager_id;
    let rewards_canister_id = test_env.rewards_canister_id;

    // Get cycles_manager balance
    let initial_cycles_manager_balance = test_env.pic.cycle_balance(cycles_manager_id);
    assert_eq!(99999999096206770, initial_cycles_manager_balance);

    // Get rewards_canister balance
    let initial_rewards_canister_balance = test_env.pic.cycle_balance(rewards_canister_id);
    assert_eq!(199987320473955, initial_rewards_canister_balance);

    // Simulate the time passing ()
    test_env.pic.advance_time(Duration::from_secs(3600000));
    tick_n_blocks(&test_env.pic, 100);

    // Check if the rewards canister has low balance
    let current_cycles_manager_balance = test_env.pic.cycle_balance(cycles_manager_id);
    assert_eq!(99999997792333604, current_cycles_manager_balance);

    let current_rewards_canister_balance = test_env.pic.cycle_balance(rewards_canister_id);
    assert_eq!(199966186559080, current_rewards_canister_balance);

    // The threshold is set up to 200_000_000_000_000
    assert!(current_rewards_canister_balance > 200_000_000_000_000);
}
