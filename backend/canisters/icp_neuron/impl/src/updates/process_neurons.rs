use ic_cdk::update;

use crate::guards::is_test_mode;

#[update(hidden = true)]
pub fn process_neurons_manual() {
    crate::jobs::process_neurons::run()
}
