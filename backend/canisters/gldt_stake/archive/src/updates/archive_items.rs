use crate::guards::caller_is_authorized;
use crate::state::mutate_state;
pub use gldt_stake_api_archive::archive_items::{
    Args as ArchiveItemsArgs, Response as ArchiveItemsResponse,
};
use ic_cdk::update;

#[update(guard = "caller_is_authorized")]
async fn archive_items(items: ArchiveItemsArgs) -> ArchiveItemsResponse {
    mutate_state(|s| s.data.archive.insert_archive_item_bulk(items))
}
