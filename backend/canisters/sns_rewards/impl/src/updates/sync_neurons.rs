// use sns_governance_canister::types::ListNeuronsResponse;
use crate::jobs::synchronise_neurons::run;
use ic_cdk::update;

// Only for development, remove after
#[update(hidden = true)]
async fn sync_neurons_manual_trigger() {
    run()
}
