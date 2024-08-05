use crate::state::{mutate_state, read_state};
use canister_time::run_now_then_interval;
use canister_tracing_macros::trace;
use std::time::Duration;
use tracing::error;
use types::CanisterId;

const INTERVAL: Duration = Duration::from_secs(1 * 60 * 60); // 1 hour

pub fn start_job() {
    run_now_then_interval(INTERVAL, run);
}

pub fn run() {
    let canister_id = read_state(|state| state.data.proposal_config.sns_governance_canister);
    ic_cdk::spawn(check_journal(canister_id));
}

#[trace]
async fn check_journal(canister_id: CanisterId) {
    // let args = ManageNeuron {};
    // match sns_governance_canister_c2c_client::manage_neuron(canister_id, args).await {
    //     Ok(response) => {}
    //     Err(e) => {
    //         error!("Failed to get SNS canisters summary: {:?}", e);
    //     }
    // }
}
