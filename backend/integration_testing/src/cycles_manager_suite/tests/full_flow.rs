use crate::client::icrc1::client::balance_of;
use crate::cycles_manager_suite::setup::default_full_flow;
use crate::utils::tick_n_blocks;
use std::time::Duration;

#[test]
fn full_flow() {
    // Initialize the tracing subscriber to capture and display logs
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let test_env = default_full_flow();

    let cycles_manager_id = test_env.cycles_manager_id;
    let cycles_burner_id = test_env.burner_canister_id;

    let icp_balance = balance_of(
        &test_env.pic,
        test_env.icp_ledger_canister_id,
        cycles_manager_id,
    );
    let cycles_balance = test_env.pic.cycle_balance(test_env.cycles_manager_id);
    // Get burner_canister balance (initially it's greater than the top_up threshold)
    let burner_canister_balance = test_env.pic.cycle_balance(cycles_burner_id);

    test_env.pic.advance_time(Duration::from_secs(5 * 60 * 60));
    tick_n_blocks(&test_env.pic, 100);

    let new_icp_balance = balance_of(
        &test_env.pic,
        test_env.icp_ledger_canister_id,
        cycles_manager_id,
    );
    let new_cycles_balance = test_env.pic.cycle_balance(test_env.cycles_manager_id);
    let new_burner_canister_balance = test_env.pic.cycle_balance(cycles_burner_id);

    let manager_status = test_env
        .pic
        .canister_status(cycles_manager_id, Some(test_env.controller));
    println!("{:?}", manager_status.unwrap().status);

    let burner_status = test_env
        .pic
        .canister_status(test_env.burner_canister_id, Some(test_env.controller));
    println!("{:?}", burner_status.unwrap().status);

    println!("icp_balance: {}", icp_balance);
    println!("new_icp_balance: {}", new_icp_balance);

    println!("cycles_balance: {}", cycles_balance);
    println!("new_cycles_balance: {}", new_cycles_balance);

    println!("burner_canister_balance: {}", burner_canister_balance);
    println!(
        "new_burner_canister_balance: {}",
        new_burner_canister_balance
    );

    assert!(new_icp_balance < icp_balance);
    assert!(new_cycles_balance > 200_000_000_000_000);
    assert!(new_burner_canister_balance > 200_000_000_000_000);
}

// icp_balance: 10_000_000_000_000_000
// new_icp_balance: 9_999_999_999_960_000 // fee ?
// cycles_balance:     599965456718486
// new_cycles_balance: 199991790698831  // should be greater than 200_000_000_000_000
// burner_canister_balance:         52061211356
// new_burner_canister_balance: 200052051717099 // should be greater than 200_000_000_000_000

// icp_balance: 100
// new_icp_balance: 100
// cycles_balance: 999965460077153
// new_cycles_balance: 199991807604068
// burner_canister_balance: 52061211356
// new_burner_canister_balance: 400052051717099
