use crate::state::read_state;
use bity_ic_canister_time::timestamp_millis;
use bity_ic_canister_tracing_macros::trace;
pub use gldt_stake_api_canister::list_all_positions::{
    Args as ListAllPositionsArgs, Response as ListAllPositionsResponse,
};
use ic_cdk::query;

const MAX_LIMIT: u64 = 100;

#[query]
#[trace]
fn list_all_positions(args: ListAllPositionsArgs) -> ListAllPositionsResponse {
    let now = timestamp_millis();

    if let Some(principal) = args.of_principal {
        read_state(|s| {
            s.data
                .stake_system
                .get_stake_position(&principal)
                .into_iter() // convert Option<T> into iterator (0 or 1 item)
                .map(|p| (p, now).into()) // convert to StakePositionResponse
                .collect()
        })
    } else {
        list_all_positions_impl(args.limit, args.skip, now)
    }
}

fn list_all_positions_impl(limit: u64, skip: u64, now: u64) -> ListAllPositionsResponse {
    let limit = limit.min(MAX_LIMIT) as usize;

    read_state(|s| {
        s.data
            .stake_system
            .stakes
            .values()
            .skip(skip as usize)
            .take(limit)
            .map(|p| (p.clone(), now).into())
            .collect()
    })
}
