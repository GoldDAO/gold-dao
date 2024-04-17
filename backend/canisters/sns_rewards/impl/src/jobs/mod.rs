pub mod synchronise_neurons;
pub mod distribute_rewards;
pub mod reserve_pool_distribution;
pub mod test;

pub(crate) fn start() {
    synchronise_neurons::start_job();
    distribute_rewards::start_job();
    test::start_job();
    // reserve_pool_distribution::start_job();
}
