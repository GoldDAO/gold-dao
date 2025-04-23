use candid::CandidType;
use gldt_stake_common::proposals::VoteType;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::ProposalId;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Args {
    pub neuron_id: String,
    pub limit: u64,
    pub skip: u64,
}

pub type Response = Vec<(ProposalId, i32, VoteType)>;
