pub use gldt_stake_api_canister::get_archive_canisters::{
    Args as GetArchiveCanistersArgs, Response as GetArchiveCanistersResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
async fn get_archive_canisters(_: GetArchiveCanistersArgs) -> GetArchiveCanistersResponse {
    read_state(|s| s.data.archive_system.get_archive_canisters())
}
