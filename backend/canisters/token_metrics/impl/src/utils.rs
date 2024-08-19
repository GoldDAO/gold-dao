use sns_governance_canister::types::ProposalData;

pub fn is_proposal_closed(proposal: &ProposalData) -> bool {
    proposal.decided_timestamp_seconds > 0
}
