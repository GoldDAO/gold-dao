use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::get_gldt_staked_history::{
    Args as GetGldtStakedHistoryArgs, Response as GetGldtStakedHistoryResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
#[trace]
fn get_all_gldt_staked_history(args: GetGldtStakedHistoryArgs) -> GetGldtStakedHistoryResponse {
    get_all_gldt_staked_history_impl(args)
}

fn get_all_gldt_staked_history_impl(
    args: GetGldtStakedHistoryArgs,
) -> GetGldtStakedHistoryResponse {
    read_state(|s| {
        s.data
            .analytics_system
            .get_staked_gldt(args.starting_day, args.limit)
    })
}
