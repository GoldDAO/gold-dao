pub use gldt_stake_api_canister::get_apy_timeseries::{
    Args as GetApyTimeseriesArgs, Response as GetApyTimeseriesResponse,
};
use ic_cdk::query;
use types::TimestampMillis;

use crate::state::read_state;

#[query]
fn get_apy_timeseries(args: GetApyTimeseriesArgs) -> GetApyTimeseriesResponse {
    get_daily_series(args)
}

fn get_daily_series(args: GetApyTimeseriesArgs) -> GetApyTimeseriesResponse {
    read_state(|s| {
        s.data
            .analytics_system
            .get_apys_rev(args.starting_day, args.limit)
    })
}
