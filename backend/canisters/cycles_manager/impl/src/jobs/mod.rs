pub mod top_up_sns_canisters;

pub use top_up_sns_canisters::sync_canister_stats;

pub(crate) fn start() {
    top_up_sns_canisters::start_job();
}
