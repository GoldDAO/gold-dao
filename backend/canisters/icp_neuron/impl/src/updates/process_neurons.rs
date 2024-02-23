use ic_cdk::update;

use crate::guards::caller_is_governance_principal;

#[update(hidden = true, guard = "caller_is_governance_principal")]
pub fn process_neurons_manual() {
    crate::jobs::process_neurons::run()
}
