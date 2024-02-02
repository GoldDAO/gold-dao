use ic_cdk::storage;
use ic_cdk_macros::pre_upgrade;

use crate::state::take_state;

#[pre_upgrade]
fn pre_upgrade() {
    // TODO: add tracing and logging
    let runtime_state = take_state();

    let stable_state = (runtime_state,);

    if let Err(err) = storage::stable_save(stable_state) {
        ic_cdk::api::trap(
            &format!("ERROR :: pre_upgrade :: failed to save stable memory. Error: {err}")
        )
    };
}
