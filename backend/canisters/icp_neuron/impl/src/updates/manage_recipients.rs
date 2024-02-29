use crate::guards::caller_is_governance_principal;
use canister_tracing_macros::trace;
use ic_cdk::update;

// method to add / remove recipients of reward distribution
#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn manage_recipients() -> () {
    manage_recipients_impl().await
}

pub(crate) async fn manage_recipients_impl() -> () {
    // TODO - implement this
    ()
}
