use sns_governance_canister::types::NeuronId;
use types::{NeuronInfo, TimestampMillis};

pub type Args = Option<usize>;
pub type Response = Vec<((NeuronId, TimestampMillis), NeuronInfo)>;
