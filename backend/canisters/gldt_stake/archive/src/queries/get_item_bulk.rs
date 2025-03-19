use crate::state::read_state;
pub use gldt_stake_api_archive::get_item_bulk::{
    Args as GetArchiveItemBulkArgs, Response as GetArchiveItemBulkResponse,
};
use ic_cdk::query;

#[query]
async fn get_item_bulk(item_ids: GetArchiveItemBulkArgs) -> GetArchiveItemBulkResponse {
    read_state(|s| s.data.archive.get_item_bulk(&item_ids))
}
