use crate::guards::caller_is_governance_principal;
use bity_ic_canister_tracing_macros::trace;
use ic_cdk::query;
use ic_cdk::update;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn set_visibility_manual_validate() -> Result<String, String> {
    Ok("No arguments to validate".to_string())
}

#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
pub fn set_visibility_manual() {
    ic_cdk::futures::spawn(crate::jobs::set_visibility::run_async());
}
