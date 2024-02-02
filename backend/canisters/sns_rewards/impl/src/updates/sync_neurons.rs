// use sns_governance_canister::types::ListNeuronsResponse;
use crate::jobs::process_neurons::update_neuron_data;
use ic_cdk::update;

#[update]
async fn sync_neurons() {
    update_neuron_data().await
}
