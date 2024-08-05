use sns_governance_canister::types::NeuronId;
use types::NeuronInfo;

pub type Args = NeuronId;
pub type Response = Option<NeuronInfo>;
