/*!
# SNS neuron maturity process

This job is responsible for processing the maturity of neurons. It is run every
epoch and processes the maturity of all neurons in this epoch. This maturity
is stored in the canister and is used to determine the rewards that a neuron
is eligible for.
*/

use crate::state::mutate_state;
use canister_time::timestamp_millis;
use tracing::info;

pub fn start_job() {
    run();
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

async fn run_async() {
    migrate_neuron_maturity_data().await;
}

pub async fn migrate_neuron_maturity_data() {
    info!("Migration started for neuron maturity data");

    mutate_state(|state| {
        state.data.maturity_history.migrate();
        state.data.migration_finished = Some(timestamp_millis());
    });
}
