use sns_governance_canister::types::NeuronId;

use crate::Empty;

pub type Args = Empty;
pub type Response = Option<Vec<NeuronId>>;
