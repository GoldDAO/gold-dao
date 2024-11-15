use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_stake_history::{
    Args as GetStakeHistoryArgs, Response as GetStakeHistoryResponse,
};
use token_metrics_api::token_data::GovHistoryEntry;

use crate::state::read_state;

#[query]
fn get_stake_history(days: GetStakeHistoryArgs) -> GetStakeHistoryResponse {
    read_state(|state| {
        let mut history = state
            .data
            .gov_stake_history
            .iter()
            .collect::<Vec<GovHistoryEntry>>()
            .clone();

        history.split_off(history.len() - days)
    })
}
