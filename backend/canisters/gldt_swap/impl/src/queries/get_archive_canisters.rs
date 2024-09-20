pub use gldt_swap_api_canister::get_archive_canisters::{
    Args as GetArchiveCanistersArgs,
    Response as GetArchiveCanistersResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
async fn get_archive_canisters(_: GetArchiveCanistersArgs) -> GetArchiveCanistersResponse {
    read_state(|s| s.data.swaps.get_archive_canisters())
}
