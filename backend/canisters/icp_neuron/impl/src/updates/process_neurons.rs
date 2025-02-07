use ic_cdk::update;

#[update(hidden = true)]
pub fn process_neurons_manual() {
    crate::jobs::process_neurons::run()
}
