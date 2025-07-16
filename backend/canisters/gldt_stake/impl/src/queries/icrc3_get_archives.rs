use crate::state::icrc3_get_archives as icrc3_get_archives_impl;
pub use gldt_stake_api_canister::icrc3_get_archives::{
    Args as GetArchivesArg, Response as GetArchivesResponse,
};
use ic_cdk::query;

#[query]
async fn icrc3_get_archives(_: GetArchivesArg) -> GetArchivesResponse {
    icrc3_get_archives_impl()
}
