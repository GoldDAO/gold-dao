// /*!
// # tarnsfers GLDT swap fees that have accumulated in the fee sub account GLDT_FEE_ACCOUNT to x every 12 hours

// */

// use crate::{ state::read_state, utils::transfer_token };
// use candid::{ Nat, Principal };
// use canister_time::{ run_interval, HOUR_IN_MS };
// use gldt_swap_api_canister::gldt::{ GLDT_FEE_ACCOUNT, GLDT_TX_FEE };
// use icrc_ledger_types::icrc1::account::{ Account, Subaccount };
// use utils::env::Environment;
// use std::time::Duration;
// use tracing::{ debug, error, info };
// use types::Milliseconds;

// const TRANSFER_INTERVAL: Milliseconds = HOUR_IN_MS * 12;

// pub fn start_job() {
//     run_interval(Duration::from_millis(TRANSFER_INTERVAL), spawn_transfer_job);
// }

// pub fn spawn_transfer_job() {
//     ic_cdk::spawn(handle_transfer_fees_job())
// }

// pub async fn handle_transfer_fees_job() {
//     debug!("FEE TRANSFER JOB - START");
//     handle_transfer_fees_job_impl().await;
//     debug!("FEE TRANSFER JOB - FINISH");
// }

// async fn handle_transfer_fees_job_impl() {
//     // get balance
//     let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);

//     // if balance is over threshhold then transfer
//     let balance = match fetch_balance_of_sub_account(gldt_ledger_id, GLDT_FEE_ACCOUNT).await {
//         Ok(amount) => amount,
//         Err(_) => {
//             return;
//         }
//     };

//     if balance <= Nat::from(GLDT_TX_FEE) {
//         // check for a result of less than 0
//         return;
//     }

//     // check we're over a threshold
//     if balance.clone() - Nat::from(GLDT_TX_FEE) < Nat::from(100_000_000_000u64) {
//         return;
//     }

//     // transfer
//     match
//         transfer_token(
//             GLDT_FEE_ACCOUNT,
//             Account {
//                 owner: Principal::anonymous(),
//                 subaccount: None,
//             },
//             gldt_ledger_id,
//             balance.clone() - Nat::from(GLDT_TX_FEE)
//         ).await
//     {
//         Ok(_) => {
//             info!("SUCCESS : {:?} GLDT fees transferred to X", balance);
//         }
//         Err(e) => {
//             error!(
//                 "ERROR : GLDT failed to transfer from fee sub account to X with error : {:?}",
//                 e
//             );
//         }
//     }
// }

// async fn fetch_balance_of_sub_account(
//     ledger_canister_id: Principal,
//     sub_account: Subaccount
// ) -> Result<Nat, String> {
//     match
//         icrc_ledger_canister_c2c_client::icrc1_balance_of(
//             ledger_canister_id,
//             &(Account {
//                 owner: read_state(|s| s.env.canister_id()),
//                 subaccount: Some(sub_account),
//             })
//         ).await
//     {
//         Ok(t) => { Ok(t) }
//         Err(e) => { Err(format!("ERROR: {:?}", e.1)) }
//     }
// }
