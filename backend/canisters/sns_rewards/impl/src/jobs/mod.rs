pub mod synchronise_neurons;
pub mod distribute_rewards;

pub(crate) fn start() {
    synchronise_neurons::start_job();
    distribute_rewards::start_job();
}