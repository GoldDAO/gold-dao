pub use gldt_stake_api_canister::get_daily_analytics::{
    Args as GetDailyAnalyticsArgs, Response as GetDailyAnalyticsResponse,
};
use ic_cdk::query;

use crate::state::read_state;

#[query]
fn get_daily_analytics(args: GetDailyAnalyticsArgs) -> GetDailyAnalyticsResponse {
    read_state(|s| {
        s.data
            .analytics_system
            .get_analytics(args.starting_day, args.limit)
    })
}
