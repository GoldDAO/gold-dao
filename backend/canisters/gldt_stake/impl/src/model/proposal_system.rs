use crate::state::{mutate_state, read_state};
use bity_ic_canister_time::{timestamp_seconds, DAY_IN_SECONDS};
use gldt_stake_common::proposals::VoteType;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::{
    manage_neuron::{Command as ManageNeuronCommand, RegisterVote},
    manage_neuron_response::Command as ManageNeuronResponseCommand,
    ListProposals, ManageNeuron, NeuronId, ProposalData, ProposalId,
};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ProposalSystem {
    pub voted_proposals: HashMap<String, HashMap<u64, (i32, VoteType)>>,
}

impl ProposalSystem {
    pub fn insert_proposal(
        &mut self,
        neuron_id: &NeuronId,
        proposal_id: &u64,
        vote: &i32,
        vote_by: &VoteType,
    ) {
        match self.voted_proposals.get_mut(neuron_id.to_string().as_str()) {
            Some(proposal_list) => {
                if !proposal_list.contains_key(proposal_id) {
                    proposal_list.insert(*proposal_id, (*vote, vote_by.clone()));
                }
            }
            None => {
                let mut new_vote = HashMap::new();
                new_vote.insert(*proposal_id, (*vote, vote_by.clone()));
                self.voted_proposals.insert(neuron_id.to_string(), new_vote);
            }
        }
    }

    pub fn get_proposal_votes_by_neuron(
        &self,
        neuron_id: &NeuronId,
    ) -> Vec<(ProposalId, i32, VoteType)> {
        if let Some(votes) = self.voted_proposals.get(neuron_id.to_string().as_str()) {
            votes
                .iter()
                .map(|(id, (vote, vote_type))| (ProposalId { id: *id }, *vote, vote_type.clone()))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn get_neuron_vote_on_specific_proposal(
        &self,
        neuron_id: &NeuronId,
        proposal_id: &u64,
    ) -> Option<i32> {
        if let Some(votes) = self.voted_proposals.get(neuron_id.to_string().as_str()) {
            for (id, (vote, _)) in votes.iter() {
                if id == proposal_id {
                    return Some(*vote);
                }
            }
        }
        None
    }
}

// Fetch all actionable proposals of the SNS
async fn fetch_actionable_proposals() -> HashMap<NeuronId, Vec<ProposalData>> {
    let limit = 10;
    let sns_governance_canister_id = read_state(|s| s.data.goldao_sns_governance_canister_id);

    let mut args = ListProposals {
        limit,
        include_reward_status: vec![1],
        before_proposal: None,
        exclude_type: vec![],
        include_status: vec![],
    };

    let mut continue_scanning = true;

    let mut actionable_proposals_per_neuron = HashMap::new();
    while continue_scanning {
        continue_scanning = false;

        match sns_governance_canister_c2c_client::list_proposals(sns_governance_canister_id, &args)
            .await
        {
            Ok(response) => {
                let number_of_received_proposals = response.proposals.len();
                if (number_of_received_proposals as u32) == limit {
                    args.before_proposal = response.proposals.last().map_or_else(
                        || {
                            error!(
                                "PROCESS PROPOSALS :: last proposal not found to continue iterating.
                                This should not be possible as the limits are checked. Stopping loop here."
                            );
                            None
                        },
                        |p| {
                            continue_scanning = true;
                            p.id
                        }
                    );
                }
                // pick out any proposal which hasn't been voted on by each controlled neuron
                for p in response.proposals.iter() {
                    // response.proposals.iter().for_each(|p| {
                    let p_id = match p.id {
                        Some(id) => id.id,
                        None => {
                            error!("PROCESS PROPOSALS :: proposal id not found");
                            continue;
                        }
                    };
                    debug!("PROCESS PROPOSALS :: proposal id : {:?}", p_id);
                    p.ballots.iter().for_each(|(neuron_id_as_string, ballot)| {
                        if let Some(neuron_id) = NeuronId::new(neuron_id_as_string) {
                            if ballot.vote == 0 {
                                if !actionable_proposals_per_neuron.contains_key(&neuron_id) {
                                    actionable_proposals_per_neuron
                                        .insert(neuron_id.clone(), vec![]);
                                }
                                if let Some(proposals) =
                                    actionable_proposals_per_neuron.get_mut(&neuron_id)
                                {
                                    proposals.push(p.clone());
                                }
                            } else {
                                // if we have already voted, we check if we already tracked the vote
                                debug!(
                                    "PROCESS PROPOSALS :: neuron id : {:?} has already voted on proposal id : {:?}",
                                    neuron_id, p.id
                                );
                                if read_state(|s| s.data.proposal_system.get_neuron_vote_on_specific_proposal(&neuron_id, &p_id)).is_none() {
                                    // if we have already voted on the proposal but we don't have it in our state, it means that we voted through an followee and we need to update our state accordingly
                                    mutate_state(|s| {
                                        s.data.proposal_system.insert_proposal(
                                            &neuron_id,
                                            &p_id,
                                            &ballot.vote,
                                            &VoteType::FolloweeVote,
                                        );
                                    });
                                    info!(
                                        "PROCESS PROPOSALS :: neuron id : {:?} has already voted on proposal id : {:?} with vote : {:?} through followee.",
                                        neuron_id, p_id, ballot.vote
                                    );
                                }
                            }
                        }
                    });
                }
            }
            Err(e) => {
                error!(
                    "SYNC proposals :: ERROR :: Failed to obtain all proposals data {:?}",
                    e
                );
            }
        }
    }
    actionable_proposals_per_neuron
}

async fn vote_if_eligible(actionable_proposals: HashMap<NeuronId, Vec<ProposalData>>) {
    let sns_governance_canister_id = read_state(|s| s.data.goldao_sns_governance_canister_id);

    for (neuron_id, proposal_list) in actionable_proposals.iter() {
        debug!("PROCESS PROPOSALS :: neuron id : {:?}", neuron_id);
        for proposal_data in proposal_list.iter() {
            debug!("PROCESS PROPOSALS :: proposal id : {:?}", proposal_data.id);

            // is already time to vote?
            // we vote within the last day of the initial voting period
            if timestamp_seconds()
                < proposal_data.proposal_creation_timestamp_seconds
                    + proposal_data.initial_voting_period_seconds
                    - DAY_IN_SECONDS
            {
                debug!("Not yet time to vote.");
                continue;
            }

            // how shall we vote?
            let mut vote = 2; // default to reject
            if let Some(latest_tally) = proposal_data.latest_tally.clone() {
                if latest_tally.yes > latest_tally.no {
                    // will only vote yes if the yes votes are more than the cast no votes. Otherwise we default to reject
                    vote = 1;
                }
            }

            // send the vote if we made it this far
            info!(
                "PROCESS PROPOSALS, SENDING VOTE :: neuron id : {:?} proposal id : {:?} vote : {:?}",
                neuron_id, proposal_data.id, vote
            );

            match sns_governance_canister_c2c_client::manage_neuron(
                sns_governance_canister_id,
                ManageNeuron {
                    subaccount: neuron_id.clone().id,
                    command: Some(ManageNeuronCommand::RegisterVote(RegisterVote {
                        proposal: proposal_data.id,
                        vote,
                    })),
                },
            )
            .await
            {
                Ok(response) => {
                    if let Some(command) = response.command {
                        match command {
                            ManageNeuronResponseCommand::RegisterVote(_) => {
                                info!("PROCESS PROPOSALS :: successfully voted :: neuron id : {:?} proposal id : {:?} vote : {:?}", neuron_id, proposal_data.id, vote);
                                mutate_state(|s| {
                                    s.data.proposal_system.insert_proposal(
                                        neuron_id,
                                        &proposal_data.id.unwrap_or(ProposalId { id: 0 }).id,
                                        &vote,
                                        &VoteType::SelfVote,
                                    );
                                });
                            }
                            ManageNeuronResponseCommand::Error(err) => {
                                warn!(
                                    "PROCESS PROPOSALS :: Failed to vote on proposal {:?} with governance error {:?}",
                                    proposal_data.id, err
                                );
                            }
                            _ => {
                                error!(
                                    "PROCESS PROPOSALS :: unexpected error :: Failed to vote on proposal {:?} with error {:?}",
                                    proposal_data.id, command
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "PROCESS PROPOSALS :: Failed to vote on proposal {:?} with error {:?}",
                        proposal_data.id, e
                    );
                }
            }
        }
    }
}

pub async fn process_proposals() {
    info!("PROCESS PROPOSALS :: start");

    let actionable_proposals = fetch_actionable_proposals().await;

    vote_if_eligible(actionable_proposals).await;

    info!("PROCESS PROPOSALS :: proposals successfully processed");
}
