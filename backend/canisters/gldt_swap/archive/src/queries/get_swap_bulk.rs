use ic_cdk::query;
pub use gldt_swap_api_archive::get_swap_bulk::{
    Args as GetArchiveSwapBulkArgs,
    Response as GetArchiveSwapBulkResponse,
};
use crate::state::read_state;

#[query]
async fn get_swap_bulk(swap_ids: GetArchiveSwapBulkArgs) -> GetArchiveSwapBulkResponse {
    read_state(|s| { s.data.archive.get_swap_bulk(&swap_ids) })
}
