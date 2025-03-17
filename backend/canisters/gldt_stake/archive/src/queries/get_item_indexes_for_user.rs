pub use gldt_stake_api_archive::get_item_indexes_for_user::{
    Args as GetItemIndexesForUserArgs, Response as GetItemIndexesForUserResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
async fn get_item_indexes_for_user(
    user_principal: GetItemIndexesForUserArgs,
) -> GetItemIndexesForUserResponse {
    read_state(|s| s.data.archive.get_item_ids_for_user(&user_principal))
}
