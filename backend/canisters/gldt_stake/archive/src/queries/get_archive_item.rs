use crate::state::read_state;
pub use gldt_stake_api_archive::get_archive_item::{
    Args as GetArchiveItemArgs, Response as GetArchiveItemResponse,
};
use ic_cdk::query;

#[query]
async fn get_archive_item(id: GetArchiveItemArgs) -> GetArchiveItemResponse {
    read_state(|s| match s.data.archive.get_item(&id) {
        Some(item) => Some((id.clone(), item.clone())),
        None => None,
    })
}
