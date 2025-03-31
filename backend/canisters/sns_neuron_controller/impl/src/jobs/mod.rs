pub mod process_ogy_neurons;
pub mod process_wtn_neurons;

pub(crate) fn start() {
    process_ogy_neurons::start_job();
    process_wtn_neurons::start_job();
}
