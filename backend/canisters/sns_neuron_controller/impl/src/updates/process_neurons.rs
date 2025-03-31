use crate::guards::caller_is_governance_principal;
use ic_cdk::update;

#[update(guard = "caller_is_governance_principal", hidden = true)]
pub fn process_neurons_manual() {
    crate::jobs::process_ogy_neurons::run();
    crate::jobs::process_wtn_neurons::run()
}
