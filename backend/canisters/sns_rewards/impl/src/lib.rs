use ic_cdk::export_candid;
use candid::Principal;
use sns_governance_canister::types::NeuronId;
use state::NeuronInfo;
use queries::GetNeuronResponse;

mod lifecycle;
mod jobs;
mod queries;
mod updates;
mod state;

export_candid!();
