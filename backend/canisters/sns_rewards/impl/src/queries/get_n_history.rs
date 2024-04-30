use ic_cdk_macros::query;
pub use sns_rewards_api_canister::get_n_history::{
    Args as GetNHistoryArgs,
    Response as GetNHistoryResponse,
};

use crate::state::read_state;

#[query(hidden = true)]
fn get_n_history(size: GetNHistoryArgs) -> GetNHistoryResponse {
    read_state(|state| { state.data.maturity_history.get(size.unwrap_or(100)) })
}
