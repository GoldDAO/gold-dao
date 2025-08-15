use crate::guards::caller_is_governance_principal;
use canister_tracing_macros::trace;
use ic_cdk::{query, update};

#[update(guard = "caller_is_governance_principal")]
pub fn synchronise_neurons() {
    crate::jobs::synchronise_neurons::run()
}
