use crate::state::read_state;
pub use gldt_swap_api_archive::get_archive_size::{
    Args as GetArchiveSizeArg, Response as GetArchiveSizeResponse,
};
use ic_cdk::query;

#[query]
async fn get_archive_size(_: GetArchiveSizeArg) -> GetArchiveSizeResponse {
    read_state(|s| s.data.archive.get_archive_size_bytes())
}
