use ic_cdk::query;
pub use gldt_swap_api_archive::get_version::{
    Args as GetVersionArg,
    Response as GetVersionResponse,
};
use crate::state::read_state;

#[query]
async fn get_version(_: GetVersionArg) -> GetVersionResponse {
    read_state(|s| s.data.version.clone())
}
