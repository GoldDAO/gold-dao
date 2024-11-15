use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_voting_participation_history::{
    Args as GetVotingParticipationArgs, Response as GetVotingParticipationHistory,
};
use utils::time::fill_missing_days;

use crate::state::read_state;

#[query]
fn get_voting_participation_history(
    args: GetVotingParticipationArgs,
) -> GetVotingParticipationHistory {
    read_state(|state| {
        let data = &state.data.voting_participation_history;

        let history = data.iter().collect();

        let mut filled_history = fill_missing_days(history, args.days, 0u64);
        filled_history.split_off(filled_history.len() - (args.days as usize))
    })
}
