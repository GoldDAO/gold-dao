pub mod claim_neuron_rewards;
pub mod manage_archive_cycles;
pub mod manage_archives;
pub mod process_proposals;
pub mod process_reward_rounds;
pub mod sync_neurons;
pub mod sync_token_usd_values;
pub mod transfer_early_unstake_fees;

pub(crate) fn start() {
    claim_neuron_rewards::start_job();
    sync_neurons::start_job();
    transfer_early_unstake_fees::start_job();
    process_proposals::start_job();
    process_reward_rounds::start_job();
    manage_archives::start_job();
    manage_archive_cycles::start_job();
    sync_token_usd_values::start_job();
}
