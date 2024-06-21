use ic_cdk_macros::query;
pub use sns_rewards_api_canister::get_random_state::{
    Args as GetRandomStateArgs,
    Response as GetRandomStateResponse,
};

use crate::state::read_state;

#[query]
fn get_random_state(size: GetRandomStateArgs) -> GetRandomStateResponse {
    read_state(|state| state.data.last_distribution_time.clone())
}
