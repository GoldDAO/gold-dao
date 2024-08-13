use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_voting_participation_history::{
    Args as GetVotingParticipationArgs,
    Response as GetVotingParticipationHistory,
};
use utils::time::fill_missing_days;

use crate::state::read_state;

#[query]
fn get_voting_participation_history(
    args: GetVotingParticipationArgs
) -> GetVotingParticipationHistory {
    let voting_participation_history = read_state(|state|
        state.data.voting_participation_history.clone()
    );
    let history = voting_participation_history.into_iter().collect();

    let mut filled_history = fill_missing_days(history, args.days, 0u64);
    let ret = filled_history.split_off(filled_history.len() - (args.days as usize));
    return ret;
}
