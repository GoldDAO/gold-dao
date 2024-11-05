use crate::guards::caller_is_authorized;
use crate::state::mutate_state;
pub use gldt_swap_api_archive::archive_swap::{
    Args as ArchiveSwapArgs, Response as ArchiveSwapResponse,
};
use ic_cdk::update;

#[update(guard = "caller_is_authorized")]
async fn archive_swap(single_swap: ArchiveSwapArgs) -> ArchiveSwapResponse {
    mutate_state(|s| s.data.archive.insert_archive_swap_bulk(vec![single_swap]))
}
