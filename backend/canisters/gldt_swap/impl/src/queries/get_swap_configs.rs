pub use gldt_swap_api_canister::get_swap_configs::{
    Args as GetSwapConfigsArgs, Response as GetSwapConfigsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
pub fn get_swap_configs(_: GetSwapConfigsArgs) -> GetSwapConfigsResponse {
    read_state(|s| s.data.swap_configs.configs.clone())
}
