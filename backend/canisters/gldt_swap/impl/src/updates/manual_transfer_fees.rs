#![allow(unused_imports)]

use crate::guards::caller_is_authorized;
use crate::state::mutate_state;
pub use gldt_swap_api_canister::manual_transfer_fees::{
    Args as ManualTransferFeesArgs, Response as ManualTransferFeesResponse,
};
use ic_cdk::update;

#[update(hidden = true, guard = "caller_is_authorized")]
async fn manual_transfer_fees(_: ManualTransferFeesArgs) -> ManualTransferFeesResponse {
    manual_transfer_fees_impl().await
}

async fn manual_transfer_fees_impl() {
    crate::jobs::transfer_fees::spawn_transfer_fees_job();
}
