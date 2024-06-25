pub mod synchronise_neurons;
pub mod distribute_rewards;
pub mod reserve_pool_distribution;
pub mod burn_job;

pub(crate) fn start() {
    synchronise_neurons::start_job();
    distribute_rewards::start_job();
    reserve_pool_distribution::start_job();
    burn_job::start_job();
}
