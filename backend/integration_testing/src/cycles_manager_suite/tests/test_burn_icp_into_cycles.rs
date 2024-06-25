use crate::client::cycles_manager;
use crate::client::icrc1::client::balance_of;
use crate::cycles_manager_suite::setup::default_burn_icp_into_cycles_test_setup;
use crate::utils::tick_n_blocks;
use ic_ledger_types::Tokens;
use std::time::Duration;

#[test]
fn icp_is_burned_into_cycles() {
    let mut test_env = default_burn_icp_into_cycles_test_setup();

    let cycles_manager_id = test_env.cycles_manager_id;

    let icp_balance = balance_of(
        &test_env.pic,
        test_env.icp_ledger_canister_id,
        cycles_manager_id,
    );
    let cycles_balance = test_env.pic.cycle_balance(test_env.cycles_manager_id);

    // Make the top up amout higher in order to initiate icp burn
    cycles_manager::update_config(
        &mut test_env.pic,
        test_env.controller,
        cycles_manager_id,
        &cycles_manager_api_canister::update_config::Args {
            max_top_up_amount: Some((cycles_balance + 100_000_000_000_000).try_into().unwrap()),
            min_cycles_balance: None,
            // NOTE: There is some limit on the amount of ICP that can be burned
            // TODO: check the limit
            icp_burn_amount: Some(Tokens::from_e8s(50_000_000)),
        },
    );

    test_env.pic.advance_time(Duration::from_secs(1 * 60 * 60));
    tick_n_blocks(&test_env.pic, 10);

    let new_icp_balance = balance_of(
        &test_env.pic,
        test_env.icp_ledger_canister_id,
        cycles_manager_id,
    );
    let new_cycles_balance = test_env.pic.cycle_balance(test_env.cycles_manager_id);

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

    assert!(new_icp_balance < icp_balance);
    assert!(
        new_cycles_balance > cycles_balance,
        "{cycles_balance} {new_cycles_balance} {}",
        cycles_manager_id
    );
}
