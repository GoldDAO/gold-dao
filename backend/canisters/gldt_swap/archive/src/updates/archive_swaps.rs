use ic_cdk::update;
pub use gldt_swap_api_archive::archive_swaps::{
    Args as ArchiveSwapsArgs,
    Response as ArchiveSwapsResponse,
};
use crate::guards::caller_is_authorized;
use crate::state::mutate_state;

#[update(guard = "caller_is_authorized")]
async fn archive_swaps(swaps: ArchiveSwapsArgs) -> ArchiveSwapsResponse {
    mutate_state(|s| s.data.archive.insert_archive_swap_bulk(swaps))
}
