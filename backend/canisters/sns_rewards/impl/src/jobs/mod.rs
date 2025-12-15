pub mod distribute_rewards;
pub mod synchronise_neurons;

pub(crate) fn start() {
    synchronise_neurons::start_job();
    distribute_rewards::start_job();
}
