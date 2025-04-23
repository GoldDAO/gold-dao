use std::collections::HashMap;

use crate::state::{mutate_state, read_state};
use canister_time::{timestamp_seconds, DAY_IN_SECONDS};
use gldt_stake_common::proposals::VoteType;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::{
    manage_neuron::{Command as ManageNeuronCommand, RegisterVote},
    ListProposals, ManageNeuron, ProposalData, ProposalId,
};
use sns_governance_canister::types::{
    manage_neuron_response::Command as ManageNeuronResponseCommand, NeuronId,
};
use tracing::{debug, info};
use tracing::{error, warn};

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
                ic_cdk::println!(
                    "Received {} proposals from SNS governance canister",
                    response.proposals.len()
                );
                ic_cdk::println!(
                    "Proposals {:?} proposals from SNS governance canister",
                    response.proposals
                );

                let number_of_received_proposals = response.proposals.len();
                if (number_of_received_proposals as u32) == limit {
                    args.before_proposal = response.proposals.last().map_or_else(
                        || {
                            ic_cdk::println!(
                                "Last proposal not found to continue iterating. Stopping loop."
                            );
                            None
                        },
                        |p| {
                            continue_scanning = true;
                            p.id
                        },
                    );
                }

                for p in response.proposals.iter() {
                    let p_id = match p.id {
                        Some(id) => id.id,
                        None => {
                            ic_cdk::println!("Proposal ID not found for a proposal. Skipping.");
                            continue;
                        }
                    };

                    ic_cdk::println!("Processing proposal ID: {:?}", p_id);
                    ic_cdk::println!("Proposal ID {:?} has {} ballots", p_id, p.ballots.len());

                    p.ballots.iter().for_each(|(neuron_id_as_string, ballot)| {
                        if let Some(neuron_id) = NeuronId::new(neuron_id_as_string) {
                            if ballot.vote == 0 {
                                ic_cdk::println!(
                                    "Neuron ID {:?} has NOT voted on proposal ID {:?}",
                                    neuron_id,
                                    p_id
                                );

                                if !actionable_proposals_per_neuron.contains_key(&neuron_id) {
                                    ic_cdk::println!(
                                        "Neuron ID {:?} not in actionable list yet. Initializing entry.",
                                        neuron_id
                                    );
                                    actionable_proposals_per_neuron.insert(neuron_id.clone(), vec![]);
                                }

                                if let Some(proposals) =
                                    actionable_proposals_per_neuron.get_mut(&neuron_id)
                                {
                                    ic_cdk::println!(
                                        "Adding proposal ID {:?} to actionable list for neuron ID {:?}",
                                        p_id,
                                        neuron_id
                                    );
                                    proposals.push(p.clone());
                                } else {
                                    ic_cdk::println!(
                                        "Failed to get mutable proposals list for neuron ID {:?}",
                                        neuron_id
                                    );
                                }
                            } else {
                                ic_cdk::println!(
                                    "Neuron ID {:?} has already voted on proposal ID {:?}",
                                    neuron_id,
                                    p.id
                                );
                                if read_state(|s| s.data.proposal_system.get_neuron_vote_on_specific_proposal(&neuron_id, &p_id)).is_none() {
                                    mutate_state(|s| {
                                        s.data.proposal_system.insert_proposal(
                                            &neuron_id,
                                            &p_id,
                                            &ballot.vote,
                                            &VoteType::FolloweeVote,
                                        );
                                    });
                                    ic_cdk::println!(
                                        "Neuron ID {:?} has already voted on proposal ID {:?} with vote {:?} through followee.",
                                        neuron_id,
                                        p_id,
                                        ballot.vote
                                    );
                                }
                            }
                        }
                    });
                }
            }
            Err(e) => {
                ic_cdk::println!(
                    "Failed to obtain all proposals data from SNS governance canister: {:?}",
                    e
                );
            }
        }
    }

    ic_cdk::println!(
        "Completed fetching actionable proposals. Total neurons with actionable proposals: {}",
        actionable_proposals_per_neuron.len()
    );

    actionable_proposals_per_neuron
}

async fn vote_if_eligible(actionable_proposals: HashMap<NeuronId, Vec<ProposalData>>) {
    let sns_governance_canister_id = read_state(|s| s.data.goldao_sns_governance_canister_id);

    for (neuron_id, proposal_list) in actionable_proposals.iter() {
        ic_cdk::println!("PROCESS PROPOSALS :: neuron id : {:?}", neuron_id);
        for proposal_data in proposal_list.iter() {
            ic_cdk::println!("PROCESS PROPOSALS :: proposal id : {:?}", proposal_data.id);

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
            ic_cdk::println!(
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
                                ic_cdk::println!("PROCESS PROPOSALS :: successfully voted :: neuron id : {:?} proposal id : {:?} vote : {:?}", neuron_id, proposal_data.id, vote);
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
                                ic_cdk::println!(
                                    "PROCESS PROPOSALS :: Failed to vote on proposal {:?} with governance error {:?}",
                                    proposal_data.id, err
                                );
                            }
                            _ => {
                                ic_cdk::println!(
                                    "PROCESS PROPOSALS :: unexpected error :: Failed to vote on proposal {:?} with error {:?}",
                                    proposal_data.id, command
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    ic_cdk::println!(
                        "PROCESS PROPOSALS :: Failed to vote on proposal {:?} with error {:?}",
                        proposal_data.id,
                        e
                    );
                }
            }
        }
    }
}

pub async fn process_proposals() {
    ic_cdk::println!("PROCESS PROPOSALS :: start");

    let actionable_proposals = fetch_actionable_proposals().await;
    ic_cdk::println!(
        "PROCESS PROPOSALS :: actionable proposals : {:?}",
        actionable_proposals
    );

    vote_if_eligible(actionable_proposals).await;

    ic_cdk::println!("PROCESS PROPOSALS :: proposals successfully processed");
}
