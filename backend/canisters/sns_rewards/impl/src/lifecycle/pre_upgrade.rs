use ic_cdk::storage;
use ic_cdk_macros::pre_upgrade;
use tracing::info;

use crate::state::take_state;

#[pre_upgrade]
fn pre_upgrade() {
    info!("Pre upgrade.");

    let runtime_state = take_state();

    let logs = canister_logger::export_logs();
    let traces = canister_logger::export_traces();

    let stable_state = (runtime_state, logs, traces);

    storage::stable_save(stable_state).unwrap();
}
