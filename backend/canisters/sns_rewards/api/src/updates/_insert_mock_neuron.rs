use candid::CandidType;
use serde::Deserialize;
use serde::Serialize;
use sns_governance_canister::types::NeuronId;
use types::NeuronInfo;

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Args {
    pub neuron_id: NeuronId,
    pub neuron_info: NeuronInfo,
}
pub type Response = Result<(), String>;
