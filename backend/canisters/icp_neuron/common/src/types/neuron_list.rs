use candid::CandidType;
use serde::Serialize;

use super::neuron_metrics::NeuronWithMetric;

#[derive(CandidType, Serialize)]
pub struct NeuronList {
    pub active: Vec<NeuronWithMetric>,
    pub spawning: Vec<u64>,
    pub disbursed: Vec<u64>,
}
