use crate::state::icrc3_supported_block_types as icrc3_supported_block_types_impl;
pub use gldt_stake_api_canister::icrc3_supported_block_types::{
    Args as GetSupportedBlockTypesArg, Response as GetSupportedBlockTypesResponse,
};
use ic_cdk::query;

#[query]
async fn icrc3_supported_block_types(
    _: GetSupportedBlockTypesArg,
) -> GetSupportedBlockTypesResponse {
    icrc3_supported_block_types_impl()
}
