use candid::{CandidType, Deserialize};
use serde::Serialize;
use sns_governance_canister::types::NeuronId;

use crate::gldt_stake_suite::setup::default_test_setup;

#[derive(Deserialize, CandidType, Serialize)]
pub struct GetNeuronRequest {
    neuron_id: NeuronId,
}

#[test]
fn inits_correctly() {
    let test_env = default_test_setup();
    // println!("{:?}", test_env);
}
