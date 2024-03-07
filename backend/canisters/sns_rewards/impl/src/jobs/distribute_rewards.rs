/*!
# SNS neuron maturity process

This job is responsible for processing the maturity of neurons. It is run every
epoch and processes the maturity of all neurons in this epoch. This maturity
is stored in the canister and is used to determine the rewards that a neuron
is eligible for.
*/

use canister_time::{ now_millis, run_now_then_interval, WEEK_IN_MS };
use sns_governance_canister::types::{ NeuronId, Neuron };
use tracing::{ debug, error, info, warn };
use std::{ collections::btree_map, time::Duration };
use types::{ Maturity, Milliseconds, NeuronInfo };

use crate::state::{ mutate_state, read_state, RuntimeState };

const DISTRIBUTION_INTERVAL: Milliseconds = WEEK_IN_MS;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(DISTRIBUTION_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(distribute_rewards())
}

pub async fn distribute_rewards() {
    let canister_id = read_state(|state| state.data.sns_governance_canister);
}


#[cfg(test)]
mod tests {
    
}
