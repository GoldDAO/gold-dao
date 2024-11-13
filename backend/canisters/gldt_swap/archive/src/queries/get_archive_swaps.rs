pub use gldt_swap_api_archive::get_archive_swaps::{
    Args as GetArchiveSwapsArgs, Response as GetArchiveSwapsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
async fn get_archive_swaps(args: GetArchiveSwapsArgs) -> GetArchiveSwapsResponse {
    let user_principal = args.user_principal;
    read_state(|s| {
        s.data
            .archive
            .get_swaps(args.start, args.limit, user_principal)
    })
}
