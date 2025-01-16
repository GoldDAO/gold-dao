pub mod claim_neuron_rewards;
pub mod process_reward_rounds;
pub mod sync_neurons;
pub mod transfer_early_unstake_fees;

pub(crate) fn start() {
    claim_neuron_rewards::start_job();
    sync_neurons::start_job();
    transfer_early_unstake_fees::start_job();
    process_reward_rounds::start_job();
}
