use crate::client::gldt_stake::get_proposal_votes_of_neuron;
use crate::gldt_stake_suite::setup::setup::GldtStakeTestEnv;
use crate::{gldt_stake_suite::setup::proposal_vote_test_setup, utils::tick_n_blocks};
use canister_time::{DAY_IN_MS, HOUR_IN_MS};
use std::time::Duration;

#[test]
fn test_voting() {
    let mut test_env = proposal_vote_test_setup();

    let GldtStakeTestEnv {
        ref mut pic,
        controller,
        gld_sns_test_env,
        gldt_stake_canister_id,
        neuron_data,
        ..
    } = test_env;
    let pic_borrowed = &pic.borrow();

    // Create a proposal using controller's neuron
    let neuron_id = crate::sns_test_env::sns_test_env::neuron_id_from_number(2);
    let result = gld_sns_test_env.submit_mock_proposal(controller, &neuron_id);

    match gld_sns_test_env.get_proposal(result).result {
        Some(proposal) => match proposal {
            sns_governance_canister::types::get_proposal_response::Result::Proposal(proposal) => {
                println!("Proposal: {:?}", proposal);
            }
            sns_governance_canister::types::get_proposal_response::Result::Error(err) => {
                panic!("Error getting proposal: {:?}", err);
            }
        },
        None => {
            panic!("Error getting proposal");
        }
    }

    pic_borrowed.advance_time(Duration::from_millis(DAY_IN_MS * 6));
    tick_n_blocks(pic_borrowed, 5);
    pic_borrowed.advance_time(Duration::from_millis(HOUR_IN_MS));
    tick_n_blocks(pic_borrowed, 5);

    match gld_sns_test_env.get_proposal(result).result {
        Some(proposal) => match proposal {
            sns_governance_canister::types::get_proposal_response::Result::Proposal(proposal) => {
                println!("Proposal: {:?}", proposal);
            }
            sns_governance_canister::types::get_proposal_response::Result::Error(err) => {
                panic!("Error getting proposal: {:?}", err);
            }
        },
        None => {
            panic!("Error getting proposal");
        }
    }

    // NOTE: wait till the last day of voting
    pic_borrowed.advance_time(Duration::from_millis(DAY_IN_MS * 23));
    tick_n_blocks(pic_borrowed, 5);
    pic_borrowed.advance_time(Duration::from_millis(HOUR_IN_MS));
    tick_n_blocks(pic_borrowed, 5);

    let neuron_id_0 = crate::sns_test_env::sns_test_env::neuron_id_from_number(0);
    let neuron_id_1 = crate::sns_test_env::sns_test_env::neuron_id_from_number(1);
    let res1 = get_proposal_votes_of_neuron(
        &pic_borrowed,
        controller,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::get_proposal_votes_of_neuron::Args {
            neuron_id: neuron_id_0.to_string(),
            limit: 10,
            skip: 0,
        },
    );
    println!("Votes_neuron_1: {:?}", res1);
    assert!(!res1.is_empty());
    for vote in res1.iter() {
        assert_eq!(
            vote.2,
            gldt_stake_common::proposals::VoteType::SelfVote,
            "VoteType is not SelfVoted"
        );
    }

    let res2 = get_proposal_votes_of_neuron(
        &pic_borrowed,
        controller,
        gldt_stake_canister_id,
        &gldt_stake_api_canister::get_proposal_votes_of_neuron::Args {
            neuron_id: neuron_id_1.to_string(),
            limit: 10,
            skip: 0,
        },
    );
    println!("Votes_neuron_2: {:?}", res2);
    assert!(!res2.is_empty());
    // Iterate over all results and verify VoteType is SelfVoted
    for vote in res2.iter() {
        assert_eq!(
            vote.2,
            gldt_stake_common::proposals::VoteType::SelfVote,
            "VoteType is not SelfVoted"
        );
    }
}
