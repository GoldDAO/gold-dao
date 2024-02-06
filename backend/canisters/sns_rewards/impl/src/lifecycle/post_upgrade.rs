use canister_tracing_macros::trace;
use ic_cdk_macros::post_upgrade;
use ic_cdk::storage;
use tracing::info;

use super::init_canister;

#[post_upgrade]
#[trace]
fn post_upgrade() {
    let (runtime_state, logs, traces) = storage::stable_restore().unwrap();

    canister_logger::init_with_logs(true, logs, traces);
    init_canister(runtime_state);

    info!("Post upgrade complete.")
}
