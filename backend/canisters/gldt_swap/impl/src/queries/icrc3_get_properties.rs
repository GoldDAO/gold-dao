use crate::state::icrc3_get_properties as icrc3_get_properties_impl;
pub use gldt_swap_api_canister::icrc3_get_properties::{
    Args as GetArchivePropsArg, Response as GetArchivePropsResponse,
};
use ic_cdk::query;

#[query]
async fn icrc3_get_properties(_: GetArchivePropsArg) -> GetArchivePropsResponse {
    icrc3_get_properties_impl()
}
