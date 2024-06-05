use crate::client::cycles_manager;
use crate::cycles_manager_suite::setup::default_test_setup;
use crate::utils::tick_n_blocks;
use candid::Nat;
use candid::{encode_one, CandidType, Principal};
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;
// use tracing_subscriber::FmtSubscriber;

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

// NOTE: Probably to test this part better, it could be useful to use pocket_ic 3.1.0
// There is an auto_progress method:
// NOTE: https://github.com/search?q=pic.auto_progress&type=code
// NOTE: example https://github.com/dfinity/ic/blob/9ab763c49cca40142edbfaad05acb35576e36396/packages/pocket-ic/tests/tests.rs#L251

// Define the test function
#[test]
fn test_cycles_management() {
    let mut test_env = default_test_setup();

    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(tracing::Level::ERROR)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Get canisters ID:
    let cycles_manager_id = test_env.cycles_manager_id;
    let cycles_burner_id = test_env.burner_canister_id;

    // Get cycles_manager balance
    let initial_cycles_manager_balance = test_env.pic.cycle_balance(cycles_manager_id);
    println!(
        "initial_cycles_manager_balance: {}",
        initial_cycles_manager_balance
    );

    // Get burner_canister balance (initially it's greater than the threshold)
    let initial_burner_canister_balance = test_env.pic.cycle_balance(cycles_burner_id);
    println!(
        "initial_burner_canister_balance: {}",
        initial_burner_canister_balance
    );

    cycles_manager::update_config(
        &mut test_env.pic,
        test_env.controller,
        cycles_manager_id,
        &cycles_manager_canister::update_config::Args {
            min_cycles_balance: Some(200_000_000_000_000),
            max_top_up_amount: Some(250_000_000_000_000),
        },
    );
    test_env.pic.tick();

    let register_canister_args = RegisterDappCanisterRequest {
        canister_id: Some(cycles_burner_id),
    };

    // Add cycles burner to the root canister dapps array
    let _ = test_env
        .pic
        .update_call(
            test_env.sns_root_canister_id,
            test_env.controller,
            "register_dapp_canister",
            encode_one(register_canister_args).unwrap(),
        )
        .unwrap();

    let _ = test_env
        .pic
        .update_call(
            test_env.sns_root_canister_id,
            test_env.controller,
            "get_sns_canisters_summary",
            encode_one(Empty {}).unwrap(),
        )
        .unwrap();
    // println!("Status: {:#?}", resp_raw);

    // NOTE: Uncomment to see the deserialized response
    // match resp_raw {
    //     WasmResult::Reply(bytebuf) => {
    //         // `bytebuf` contains the deserialized byte buffer
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

    // test_env.pic.advance_time(Duration::from_secs(15 * 60));
    // tick_n_blocks(&test_env.pic, 10);

    let current_cycles_manager_balance = test_env.pic.cycle_balance(cycles_manager_id);
    println!(
        "current_cycles_manager_balance: {}",
        current_cycles_manager_balance
    );

    // Check if the burner canister has low balance
    let current_burner_canister_balance = test_env.pic.cycle_balance(cycles_burner_id);
    println!(
        "current_burner_canister_balance: {}",
        current_burner_canister_balance
    );

    // Assert that the final balance is bigger that the threshold
    assert!(current_burner_canister_balance > 200_000_000_000_000);
}
