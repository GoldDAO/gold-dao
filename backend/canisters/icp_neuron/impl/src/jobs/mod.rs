pub mod process_neurons;
pub mod set_visibility;

pub(crate) fn start() {
    process_neurons::start_job();
    set_visibility::start_job();
}
