use crate::jobs::sync_canister_stats;
use crate::state::read_state;
pub use cycles_manager_api_canister::get_canisters_summary::Response as CanistersSummaryResponse;
use ic_cdk_macros::update;

#[update]
async fn get_canisters_summary() -> CanistersSummaryResponse {
    let sns_root_canister = read_state(|state: &crate::state::State| state.data.sns_root_canister);

    match sync_canister_stats(sns_root_canister).await {
        Ok(_) => CanistersSummaryResponse::Success(read_state(|state| state.metrics().canisters)),
        Err(_) => CanistersSummaryResponse::FailedGetCanisterSummary,
    }
}
