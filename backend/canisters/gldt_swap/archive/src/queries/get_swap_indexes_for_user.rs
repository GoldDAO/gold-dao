pub use gldt_swap_api_archive::get_swap_indexes_for_user::{
    Args as GetSwapIndexesForUserArgs,
    Response as GetSwapIndexesForUserResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
async fn get_swap_indexes_for_user(
    user_principal: GetSwapIndexesForUserArgs
) -> GetSwapIndexesForUserResponse {
    read_state(|s| s.data.archive.get_swap_ids_for_user(&user_principal))
}
