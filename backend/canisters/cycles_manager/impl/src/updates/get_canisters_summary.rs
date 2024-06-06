use crate::guards::caller_is_governance_principal;
use crate::state::read_state;
use cycles_manager_api_canister::get_canisters_summary::Response as CanistersSummaryResponse;
use ic_cdk_macros::update;

#[update(guard = "caller_is_governance_principal", hidden = true)]
fn get_canisters_summary() -> CanistersSummaryResponse {
    // Run the job again to get the response from the SNS root canister. Probably, there should be a more efficient way to do this.
    // TODO: Find a better way to amortize this
    crate::jobs::top_up_sns_canisters::run();
    read_state(|state| state.metrics().canisters)
}
