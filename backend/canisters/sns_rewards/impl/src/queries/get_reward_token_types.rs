use ic_cdk_macros::query;
pub use sns_rewards_api_canister::get_reward_token_types::Response as GetRewardTokenTypesResponse;
use crate::state::read_state;

#[query(hidden = true)]
fn get_reward_token_types() -> GetRewardTokenTypesResponse {
    read_state(|state| { state.data.tokens.clone() })
}
