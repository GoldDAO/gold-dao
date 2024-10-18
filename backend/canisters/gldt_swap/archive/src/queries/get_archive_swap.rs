use ic_cdk::query;
pub use gldt_swap_api_archive::get_archive_swap::{
    Args as GetArchiveSwapArgs,
    Response as GetArchiveSwapResponse,
};
use crate::state::read_state;

#[query]
async fn get_archive_swap(arg: GetArchiveSwapArgs) -> GetArchiveSwapResponse {
    read_state(|s| {
        let swap = s.data.archive.get_swap(&arg);
        match swap {
            Some(swap_info) => Some((swap_info.get_swap_id(), swap_info.clone())),

            None => None,
        }
    })
}
