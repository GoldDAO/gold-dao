pub mod allocate_rewards;
#[cfg(feature = "inttest")]
pub mod claim_neuron_rewards;
pub mod process_empty_stake_positions;
// pub mod process_proposals;
pub mod process_rewards;
pub mod sync_neurons;
pub mod sync_token_usd_values;
pub mod transfer_instant_dissolve_fees;

pub(crate) fn start() {
    process_empty_stake_positions::start_job();
    #[cfg(feature = "inttest")]
    claim_neuron_rewards::start_job();
    sync_neurons::start_job();
    transfer_instant_dissolve_fees::start_job();
    // process_proposals::start_job();
    allocate_rewards::start_job();
    sync_token_usd_values::start_job();
}
