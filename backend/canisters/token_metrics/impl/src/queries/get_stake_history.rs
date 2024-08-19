use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_stake_history::{
    Response as GetStakeHistoryResponse,
    Args as GetStakeHistoryArgs,
};

use crate::state::read_state;

#[query]
fn get_stake_history(days: GetStakeHistoryArgs) -> GetStakeHistoryResponse {
    let mut history = read_state(|state| state.data.gov_stake_history.clone());
    let ret = history.split_off(history.len() - days);
    return ret;
}
