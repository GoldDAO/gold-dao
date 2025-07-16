use crate::state::icrc3_get_blocks as icrc3_get_blocks_impl;
pub use gldt_stake_api_canister::queries::icrc3_get_blocks::{
    Args as GetBlocksArg, Response as GetBlocksResponse,
};
use ic_cdk::query;
pub use icrc_ledger_types::icrc3::blocks::GetBlocksResult;

#[query]
fn icrc3_get_blocks(args: GetBlocksArg) -> GetBlocksResult {
    icrc3_get_blocks_impl(args)
}
