use std::collections::BTreeMap;
use types::NeuronInfo;

pub type Args = ();
pub type Response = BTreeMap<sns_governance_canister::types::NeuronId, NeuronInfo>;
