use crate::guards::caller_is_authorized;
use crate::state::mutate_state;
pub use gldt_stake_api_archive::archive_item::{
    Args as ArchiveItemArgs, Response as ArchiveItemResponse,
};
use ic_cdk::update;

#[update(guard = "caller_is_authorized")]
async fn archive_item(single_item: ArchiveItemArgs) -> ArchiveItemResponse {
    mutate_state(|s| s.data.archive.insert_archive_item_bulk(vec![single_item]))
}
