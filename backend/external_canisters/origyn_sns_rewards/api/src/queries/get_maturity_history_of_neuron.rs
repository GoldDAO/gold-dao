use candid::CandidType;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::NeuronId;
use types::{NeuronInfo, TimestampMillis};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub neuron_id: NeuronId,
    pub size: Option<usize>,
}

pub type Response = Vec<(TimestampMillis, NeuronInfo)>;
