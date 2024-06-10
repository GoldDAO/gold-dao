use crate::client::cycles_manager;
use crate::cycles_manager_suite::setup::default_test_setup;
use crate::utils::tick_n_blocks;
use candid::Nat;
use candid::{encode_one, CandidType, Principal};
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

#[derive(CandidType, Serialize, Deserialize)]
pub struct Empty {}

#[derive(CandidType, Deserialize, Debug)]
pub struct RegisterDappCanisterRequest {
    pub canister_id: Option<Principal>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Response {
    pub root: Option<CanisterSummary>,
    pub governance: Option<CanisterSummary>,
    pub ledger: Option<CanisterSummary>,
    pub swap: Option<CanisterSummary>,
    pub dapps: Vec<CanisterSummary>,
    pub archives: Vec<CanisterSummary>,
    pub index: Option<CanisterSummary>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CanisterSummary {
    pub canister_id: Option<Principal>,
    pub status: Option<CanisterStatusResult>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CanisterStatusResult {
    pub cycles: Nat,
}

// Define the test function
#[test]
fn test_cycles_management() {
    let mut test_env = default_test_setup();

    // Get canisters ID:
    let cycles_manager_id = test_env.cycles_manager_id;
    let cycles_burner_id = test_env.burner_canister_id;

    // Get cycles_manager balance
    let initial_cycles_manager_balance = test_env.pic.cycle_balance(cycles_manager_id);
    println!(
        "initial_cycles_manager_balance: {}",
        initial_cycles_manager_balance
    );

    // Get burner_canister balance (initially it's greater than the top_up threshold)
    let initial_burner_canister_balance = test_env.pic.cycle_balance(cycles_burner_id);
    println!(
        "initial_burner_canister_balance: {}",
        initial_burner_canister_balance
    );

    cycles_manager::update_config(
        &mut test_env.pic,
        test_env.controller,
        cycles_manager_id,
        &cycles_manager_api_canister::update_config::Args {
            max_top_up_amount: Some(20_000_000_000_000),
            min_cycles_balance: Some(10_000_000_000_000),
        },
    );
    test_env.pic.tick();

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

    // NOTE: Uncomment to see the deserialized get_sns_canisters_summary response
    // let resp_raw = test_env
    //     .pic
    //     .update_call(
    //         test_env.sns_root_canister_id,
    //         test_env.controller,
    //         "get_sns_canisters_summary",
    //         encode_one(Empty {}).unwrap(),
    //     )
    //     .unwrap();
    //
    // match resp_raw {
    //     WasmResult::Reply(bytebuf) => {
    //         // bytebuf contains the deserialized byte buffer
    //         // https://github.com/TaxLintDAO/taxlint/blob/master/backend/i_test/src/client/mod.rs#L130
    //         let data: Response = candid::decode_one(&bytebuf).unwrap();
    //         println!("Deserialized data: {:#?}", data);
    //     }
    //     WasmResult::Reject(reason) => {
    //         // Handle rejection
    //         println!("Rejected: {}", reason);
    //     }
    // }

    test_env.pic.advance_time(Duration::from_secs(5 * 60 * 60)); // 20 days
    tick_n_blocks(&test_env.pic, 10);

    // Get cycles_manager balance
    let current_cycles_manager_balance = test_env.pic.cycle_balance(cycles_manager_id);
    println!(
        "current_cycles_manager_balance: {}",
        current_cycles_manager_balance
    );

    // Check if the burner canister was topped up
    let current_burner_canister_balance = test_env.pic.cycle_balance(cycles_burner_id);
    println!(
        "current_burner_canister_balance: {}",
        current_burner_canister_balance
    );

    // Assert that the final balance is bigger that the threshold
    assert!(current_burner_canister_balance > 10_000_000_000_000);
}
