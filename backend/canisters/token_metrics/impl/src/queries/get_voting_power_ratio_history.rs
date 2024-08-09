use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_voting_power_ratio_history::{
    Args as GetVotingPowerRatioHistoryArgs,
    Response as GetVotingPowerRatioHistoryResponse,
};
use crate::state::read_state;

#[query]
fn get_voting_power_ratio_history(
    args: GetVotingPowerRatioHistoryArgs
) -> GetVotingPowerRatioHistoryResponse {
    let mut history = read_state(|state| state.data.voting_power_ratio_history.clone());
    history.split_off(history.len() - (args.days as usize))
}
