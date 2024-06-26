use candid::Principal;
use sns_governance_canister::types::NeuronId;

pub type Args = Option<Principal>;
pub type Response = Option<Vec<NeuronId>>;
