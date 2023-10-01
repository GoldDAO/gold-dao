use candid::Principal;
use canistergeek_ic_rust::logger::log_message;

use crate::declarations::icrc1;

#[allow(dead_code)]
pub async fn dummy_await() -> Result<(), String> {
    let gldt_ledger_canister_id = Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").unwrap_or(
        Principal::anonymous()
    );
    let service_ledger = icrc1::Service(gldt_ledger_canister_id);
    log_message(
        format!("Sending dummy await - before time {}", ic_cdk::api::time() / 1_000_000_000)
    );
    let _ = service_ledger.icrc1_minting_account().await;
    log_message(
        format!("Received dummy await - after time {}", ic_cdk::api::time() / 1_000_000_000)
    );
    Ok(())
}
