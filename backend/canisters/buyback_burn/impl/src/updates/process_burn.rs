use crate::guards::caller_is_governance_principal;
use ic_cdk::update;

#[update(guard = "caller_is_governance_principal", hidden = true)]
pub fn process_burn() {
    crate::jobs::burn_tokens::run()
}
