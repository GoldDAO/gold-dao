use canister_tracing_macros::trace;

pub use gldt_stake_api_canister::queries::get_proposal_votes_of_neuron::{
    Args as GetProposalVotesByNeuronArgs, Response as GetProposalVotesByNeuronResponse,
};
use ic_cdk::query;
use sns_governance_canister::types::NeuronId;

use crate::state::read_state;

#[query]
#[trace]
async fn get_proposal_votes_of_neuron(
    args: GetProposalVotesByNeuronArgs,
) -> GetProposalVotesByNeuronResponse {
    get_proposal_votes_of_neuron_impl(args).await
}

async fn get_proposal_votes_of_neuron_impl(
    args: GetProposalVotesByNeuronArgs,
) -> GetProposalVotesByNeuronResponse {
    let neuron_id = NeuronId::new(&args.neuron_id).unwrap();
    read_state(|s| {
        s.data
            .proposal_system
            .get_proposal_votes_by_neuron(&neuron_id)
    })
}
