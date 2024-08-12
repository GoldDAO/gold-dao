use canister_time::{ run_now_then_interval, timestamp_seconds };
use sns_governance_canister::types::{ get_proposal_response, ProposalData, ProposalId };
use token_metrics_api::token_data::VotingHistoryCalculations;
use std::time::Duration;
use tracing::{ debug, error, info };
use types::Milliseconds;

use crate::{ state::{ mutate_state, read_state, RuntimeState }, utils::is_proposal_closed };

// every 5 minutes
const SYNC_PROPOSALS_STATS_INTERVAL: Milliseconds = 5 * 60 * 1_000;

pub fn start_job() {
    debug!("Starting the proposals metrics sync job..");
    run_now_then_interval(Duration::from_millis(SYNC_PROPOSALS_STATS_INTERVAL), run)
}

pub fn run() {
    ic_cdk::spawn(sync_proposals_metrics_data());
    ic_cdk::spawn(recheck_ongoing_proposals());
}

pub async fn sync_proposals_metrics_data() {
    let canister_id = read_state(|state| state.data.sns_governance_canister);
    let last_synced_proposal_id = read_state(|state| state.data.sync_info.last_synced_proposal_id);
    let mut number_of_scanned_proposals = 0;
    let mut continue_scanning = true;

    let mut args = sns_governance_canister::list_proposals::Args {
        limit: 50,
        before_proposal: None,
        exclude_type: Vec::new(),
        include_status: Vec::new(),
        include_reward_status: Vec::new(),
    };

    while continue_scanning {
        continue_scanning = false;

        match sns_governance_canister_c2c_client::list_proposals(canister_id, &args).await {
            Ok(response) => {
                let number_of_received_proposals = response.proposals.len();
                if number_of_received_proposals == 0 {
                    break;
                }

                match response.proposals.first() {
                    Some(first_proposal_in_batch) => {
                        let first_id = first_proposal_in_batch.id.unwrap_or(ProposalId { id: 0 });
                        let last_synced_id = last_synced_proposal_id.unwrap_or(ProposalId {
                            id: 0,
                        });
                        if first_id.id > last_synced_id.id {
                            mutate_state(|state| {
                                state.data.sync_info.last_synced_proposal_id = Some(first_id);
                            });
                        }
                    }
                    None => {}
                }

                for proposal in &response.proposals {
                    if let Some(last_from_state) = &last_synced_proposal_id {
                        match proposal.id {
                            Some(proposal_id) => {
                                if proposal_id.id <= last_from_state.id {
                                    // Get out of the loop if we reached a proposal we already scanned
                                    break;
                                } else {
                                    args.before_proposal = Some(proposal_id);
                                }
                            }
                            None => {
                                // Get out of the loop if we don't have an id
                                break;
                            }
                        }
                    }
                    analyze_proposal(proposal);
                    number_of_scanned_proposals += 1;
                }

                if number_of_received_proposals == (args.limit as usize) {
                    continue_scanning = true;
                }
            }
            Err(err) => {
                let error_message = format!("{err:?}");
                error!(?error_message, "Error fetching proposal data");
            }
        }
    }

    info!("Successfully scanned {number_of_scanned_proposals} proposals.");

    mutate_state(|state| {
        state.data.sync_info.last_synced_number_of_proposals = number_of_scanned_proposals;
    });

    info!("Voting metrics updated successfully.");
}

pub async fn recheck_ongoing_proposals() {
    let ongoing_proposals = read_state(|state| state.data.sync_info.ongoing_proposals.clone());
    let governance_canister_id = read_state(|state| state.data.sns_governance_canister);

    for proposal_id in ongoing_proposals {
        let args = sns_governance_canister::list_proposals::Args {
            before_proposal: Some(ProposalId { id: proposal_id.id + 1 }),
            limit: 1,
            exclude_type: Vec::new(),
            include_status: Vec::new(),
            include_reward_status: Vec::new(),
        };
        match
            sns_governance_canister_c2c_client::list_proposals(governance_canister_id, &args).await
        {
            Ok(response) => {
                let returned_proposal = response.proposals.first();
                match returned_proposal {
                    Some(proposal) => {
                        if is_proposal_closed(&proposal) {
                            mutate_state(|state| {
                                let ongoing_proposals = &mut state.data.sync_info.ongoing_proposals;
                                if let Some(this_proposal_id) = proposal.id {
                                    if
                                        let Some(pos) = ongoing_proposals
                                            .iter()
                                            .position(|id| id == &this_proposal_id)
                                    {
                                        ongoing_proposals.remove(pos);
                                    }
                                }

                                update_proposals_metrics(state, &proposal);
                            });
                        }
                    }
                    None => (),
                }
            }
            Err(e) => {
                let err_msg = format!("{e:?}");
                error!("recheck_ongoing_proposals -> {err_msg:?}");
            }
        }
    }
}

pub fn analyze_proposal(proposal: &ProposalData) {
    mutate_state(|state| {
        update_proposals_metrics(state, proposal);
    });

    mutate_state(|state| {
        let ongoing_proposals = &mut state.data.sync_info.ongoing_proposals;

        // If proposal is executed, we update the metrics
        // else push it to a vector to be checked later
        if !is_proposal_closed(proposal) {
            if let Some(pid) = proposal.id {
                ongoing_proposals.push(pid);
            }
        }
    });
}

pub fn update_proposals_metrics(state: &mut RuntimeState, proposal: &ProposalData) {
    let metrics = &mut state.data.porposals_metrics;
    let metrics_calculations = &mut state.data.proposals_metrics_calculations;

    let this_proposal_id = proposal.id.unwrap_or(ProposalId { id: 0 }).id;
    if this_proposal_id > metrics.total_proposals {
        metrics.total_proposals = this_proposal_id;
    }

    if is_proposal_closed(proposal) {
        match proposal.latest_tally.clone() {
            Some(tally) => {
                let this_proposal_participation =
                    (((tally.yes as f64) + (tally.no as f64)) / (tally.total as f64)) * 100.0;
                println!("{this_proposal_participation:?}");
                if tally.total > metrics.total_voting_power {
                    metrics.total_voting_power = tally.total;
                }
                metrics_calculations.cumulative_voting_power += tally.total;

                // Update cumulative voting participation and count of valid tallies
                metrics_calculations.cumulative_voting_participation += this_proposal_participation;
                metrics_calculations.valid_tally_count += 1;

                // Update the average voting participation
                metrics.average_voting_participation = (((
                    metrics_calculations.cumulative_voting_participation as f64
                ) /
                    (metrics_calculations.valid_tally_count as f64)) *
                    100.0) as u64;

                // Update the average voting power
                metrics.average_voting_power =
                    metrics_calculations.cumulative_voting_power / metrics.total_proposals;

                update_voting_history(state, proposal, this_proposal_participation);
            }
            None => {}
        }
    }
}

fn update_voting_history(state: &mut RuntimeState, proposal: &ProposalData, participation: f64) {
    let voting_history_calculations = &mut state.data.voting_participation_history_calculations;
    let voting_history = &mut state.data.voting_participation_history;

    // Update the daily voting_participation_history to include this proposal
    let day_of_proposal = proposal.proposal_creation_timestamp_seconds / 86400;

    let updated_voting_calculations = voting_history_calculations
        .entry(day_of_proposal)
        .and_modify(|value| {
            value.valid_tally_count += 1;
            value.cumulative_voting_participation += participation;
        })
        .or_insert(VotingHistoryCalculations {
            cumulative_voting_participation: participation,
            valid_tally_count: 1,
        });

    let new_voting_history_value = (((
        updated_voting_calculations.cumulative_voting_participation as f64
    ) /
        (updated_voting_calculations.valid_tally_count as f64)) *
        100.0) as u64;

    voting_history
        .entry(day_of_proposal)
        .and_modify(|value| {
            *value = new_voting_history_value;
        })
        .or_insert(new_voting_history_value);
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use canister_time::timestamp_seconds;
    use sns_governance_canister::types::{
        neuron::DissolveState,
        Neuron,
        NeuronId,
        ProposalData,
        ProposalId,
        Tally,
    };
    use types::{ CanisterId, NeuronInfo };
    use utils::env::CanisterEnv;

    use crate::{
        jobs::sync_proposals_stats::analyze_proposal,
        state::{ init_state, mutate_state, read_state, Data, RuntimeState },
    };

    fn init_runtime_state() {
        let env = CanisterEnv::new(true);
        let data = Data::new(
            CanisterId::anonymous(),
            CanisterId::anonymous(),
            CanisterId::anonymous(),
            CanisterId::anonymous(),
            "aaaa-aa.00..1".to_string(),
            Vec::new()
        );
        init_state(RuntimeState::new(env.clone(), data));
    }

    use super::update_proposals_metrics;

    #[test]
    fn test_update_proposals_metrics() {
        init_runtime_state();

        // Proposal 1
        let mut proposal_1 = ProposalData::default();
        proposal_1.id = Some(ProposalId {
            id: 1,
        });
        proposal_1.proposal_creation_timestamp_seconds = 86_400;
        proposal_1.decided_timestamp_seconds = 86_400;
        proposal_1.latest_tally = Some(Tally {
            timestamp_seconds: 86_400,
            yes: 10,
            no: 3,
            total: 50,
        });

        mutate_state(|state| {
            update_proposals_metrics(state, &proposal_1);
        });

        let proposals_metrics = read_state(|state| state.data.porposals_metrics.clone());
        let voting_participation_history = read_state(|state|
            state.data.voting_participation_history.clone()
        );

        assert_eq!(proposals_metrics.total_proposals, 1);
        assert_eq!(proposals_metrics.total_voting_power, 50);
        assert_eq!(proposals_metrics.average_voting_power, 50);
        // avg = (10 + 3) / 50) = 0.26
        assert_eq!(proposals_metrics.average_voting_participation, 2600u64);
        assert_eq!(voting_participation_history.get(&1), Some(&2600u64));

        // Proposal 2
        let mut proposal_2 = ProposalData::default();
        proposal_2.id = Some(ProposalId {
            id: 2,
        });
        proposal_2.proposal_creation_timestamp_seconds = 2 * 86_400;
        proposal_2.decided_timestamp_seconds = 2 * 86_400;
        proposal_2.latest_tally = Some(Tally {
            timestamp_seconds: 2 * 86_400,
            yes: 20,
            no: 4,
            total: 60,
        });

        mutate_state(|state| {
            update_proposals_metrics(state, &proposal_2);
        });

        let proposals_metrics = read_state(|state| state.data.porposals_metrics.clone());
        let voting_participation_history = read_state(|state|
            state.data.voting_participation_history.clone()
        );

        assert_eq!(proposals_metrics.total_proposals, 2);
        assert_eq!(proposals_metrics.total_voting_power, 60);
        // avg = 50 + 60 / 2 = 55
        assert_eq!(proposals_metrics.average_voting_power, 55);
        // avg = ((10 + 3) / 50 + (20 + 4) / 60) / 2 = 0.33
        assert_eq!(proposals_metrics.average_voting_participation, 3300u64);

        // For this day = 20 + 4 / 60 = 0.4
        assert_eq!(voting_participation_history.get(&2), Some(&4000u64));

        // Proposal 3
        let mut proposal_3 = ProposalData::default();
        proposal_3.id = Some(ProposalId {
            id: 3,
        });
        proposal_3.proposal_creation_timestamp_seconds = 3 * 86_400;
        proposal_3.decided_timestamp_seconds = 3 * 86_400;
        proposal_3.latest_tally = Some(Tally {
            timestamp_seconds: 3 * 86_400,
            yes: 45,
            no: 10,
            total: 100,
        });

        mutate_state(|state| {
            update_proposals_metrics(state, &proposal_3);
        });

        let proposals_metrics = read_state(|state| state.data.porposals_metrics.clone());
        let voting_participation_history = read_state(|state|
            state.data.voting_participation_history.clone()
        );

        assert_eq!(proposals_metrics.total_proposals, 3);
        assert_eq!(proposals_metrics.total_voting_power, 100);
        // avg = 50 + 60 + 100 / 3 = 70
        assert_eq!(proposals_metrics.average_voting_power, 70);
        // avg = ((10 + 3) / 50 + (20 + 4) / 60 + (45 + 10) / 100) / 3 = 0.4033333333333333
        assert_eq!(proposals_metrics.average_voting_participation, 4033u64);

        // For this day = 45 + 10 / 100 = 0.55
        assert_eq!(voting_participation_history.get(&3), Some(&5500u64));

        // Proposal 4 - still open
        let mut proposal_4 = ProposalData::default();
        proposal_4.id = Some(ProposalId {
            id: 4,
        });
        proposal_4.proposal_creation_timestamp_seconds = 4 * 86_400;
        proposal_4.decided_timestamp_seconds = 0;
        proposal_4.latest_tally = Some(Tally {
            timestamp_seconds: 4 * 86_400,
            yes: 90,
            no: 10,
            total: 150,
        });

        analyze_proposal(&proposal_4);

        let ongoing_proposals = read_state(|state| state.data.sync_info.ongoing_proposals.clone());
        assert_eq!(
            ongoing_proposals.contains(
                &(ProposalId {
                    id: 4,
                })
            ),
            true
        );
        let proposals_metrics = read_state(|state| state.data.porposals_metrics.clone());

        assert_eq!(proposals_metrics.total_proposals, 4);
        // Expect the other values to be the same as before
        assert_eq!(proposals_metrics.total_voting_power, 100);
        // avg = 50 + 60 + 100 / 3 = 70
        assert_eq!(proposals_metrics.average_voting_power, 70);
        // avg = ((10 + 3) / 50 + (20 + 4) / 60 + (45 + 10) / 100) / 3 = 0.4033333333333333
        assert_eq!(proposals_metrics.average_voting_participation, 4033u64);

        // Close it and re-analyze the proposal
        proposal_4.decided_timestamp_seconds = 4 * 86_400;
        mutate_state(|state| {
            update_proposals_metrics(state, &proposal_4);
        });

        let proposals_metrics = read_state(|state| state.data.porposals_metrics.clone());
        assert_eq!(proposals_metrics.total_proposals, 4);
        // Expect the other values to be the same as before
        assert_eq!(proposals_metrics.total_voting_power, 150);
        // avg = 50 + 60 + 100 + 150 / 4 = 70
        assert_eq!(proposals_metrics.average_voting_power, 90);
        // avg = ((10 + 3) / 50 + (20 + 4) / 60 + (45 + 10) / 100 + (90 + 10) / 150) / 4 = 0.4033333333333333
        assert_eq!(proposals_metrics.average_voting_participation, 4691u64);

        // Proposal 5 - same day as proposal 4
        let mut proposal_5 = ProposalData::default();
        proposal_5.id = Some(ProposalId {
            id: 5,
        });
        proposal_5.proposal_creation_timestamp_seconds = 4 * 86_400;
        proposal_5.decided_timestamp_seconds = 4 * 86_400;
        proposal_5.latest_tally = Some(Tally {
            timestamp_seconds: 4 * 86_400,
            yes: 60,
            no: 20,
            total: 150,
        });
        mutate_state(|state| {
            update_proposals_metrics(state, &proposal_5);
        });
        let voting_participation_history = read_state(|state|
            state.data.voting_participation_history.clone()
        );
        // For day 4: avg between proposal 4 and proposal 5
        // = ((90 + 10) / 150 + (60 + 20) / 150) / 2 = 0.6
        assert_eq!(voting_participation_history.get(&4), Some(&6000u64));
    }
}
