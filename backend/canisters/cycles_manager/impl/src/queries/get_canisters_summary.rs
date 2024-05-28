use crate::guards::caller_is_governance_principal;
use crate::model::canisters::CanisterMetrics;
use crate::state::read_state;
use ic_cdk::update;
use ic_cdk_macros::query;

#[update(guard = "caller_is_governance_principal", hidden = true)]
fn get_canisters_summary() -> Vec<CanisterMetrics> {
    crate::jobs::top_up_sns_canisters::run();
    read_state(|state| state.metrics().canisters)
}
