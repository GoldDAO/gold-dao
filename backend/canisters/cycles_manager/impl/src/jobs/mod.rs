pub mod burn_icp_into_cycles;
pub mod top_up_sns_canisters;

pub use top_up_sns_canisters::sync_canister_stats;

pub(crate) fn start() {
    burn_icp_into_cycles::start_job();
    top_up_sns_canisters::start_job();
}
