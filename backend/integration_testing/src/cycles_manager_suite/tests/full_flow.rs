use crate::client::icrc1::client::{balance_of, transfer};
use crate::cycles_manager_suite::setup::default_full_flow;
use crate::cycles_manager_suite::tests::test_top_up::RegisterDappCanisterRequest;
use crate::utils::tick_n_blocks;
use candid::encode_one;
use std::time::Duration;

#[test]
fn full_flow() {
    let mut test_env = default_full_flow();

    let cycles_manager_id = test_env.cycles_manager_id;
    let cycles_burner_id = test_env.burner_canister_id;
    let init_cycles_balance = test_env.pic.cycle_balance(test_env.cycles_manager_id);

    // Arguments to register dapp in the sns_root_canister
    let register_canister_args = RegisterDappCanisterRequest {
        canister_id: Some(cycles_burner_id),
    };

    // Add cycles burner to the sns_root canister dapps array
    let _ = test_env
        .pic
        .update_call(
            test_env.sns_root_canister_id,
            test_env.controller,
            "register_dapp_canister",
            encode_one(register_canister_args).unwrap(),
        )
        .unwrap();

    let _ = transfer(
        &mut test_env.pic,
        test_env.controller,
        test_env.icp_ledger_canister_id,
        None,
        cycles_manager_id,
        10_000_000_000_000_000,
    )
    .unwrap();

    let icp_balance = balance_of(
        &test_env.pic,
        test_env.icp_ledger_canister_id,
        cycles_manager_id,
    );
    let cycles_balance = test_env.pic.cycle_balance(test_env.cycles_manager_id);
    // Get burner_canister balance (initially it's greater than the top_up threshold)
    let initial_burner_canister_balance = test_env.pic.cycle_balance(cycles_burner_id);

    test_env.pic.advance_time(Duration::from_secs(5 * 60 * 60));
    // test_env.pic.advance_time(Duration::from_secs(60 * 60));
    tick_n_blocks(&test_env.pic, 100);

    let new_icp_balance = balance_of(
        &test_env.pic,
        test_env.icp_ledger_canister_id,
        cycles_manager_id,
    );
    let new_cycles_balance = test_env.pic.cycle_balance(test_env.cycles_manager_id);
    // Check if the burner canister was topped up
    let current_burner_canister_balance = test_env.pic.cycle_balance(cycles_burner_id);

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

    println!("init_cycles_balance: {}", init_cycles_balance);
    println!("cycles_balance: {}", cycles_balance);
    println!("new_cycles_balance: {}", new_cycles_balance);

    println!(
        "initial_burner_canister_balance: {}",
        initial_burner_canister_balance
    );
    println!(
        "current_burner_canister_balance: {}",
        current_burner_canister_balance
    );

    assert!(new_icp_balance < icp_balance);
    assert!(new_cycles_balance > 200_000_000_000_000);
    assert!(current_burner_canister_balance > 200_000_000_000_000);
}
