use nns_governance_canister::types::Neuron;
use serde::{Deserialize, Serialize};
use types::TimestampMillis;

#[derive(Serialize, Deserialize, Default)]
pub struct Neurons {
    pub timestamp: TimestampMillis,
    pub active_neurons: Vec<Neuron>,
    pub spawning_neurons: Vec<Neuron>,
    pub disbursed_neurons: Vec<u64>,
}
