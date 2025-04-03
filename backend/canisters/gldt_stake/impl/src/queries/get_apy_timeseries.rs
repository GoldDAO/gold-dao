use std::collections::HashMap;

use candid::Nat;
use canister_time::{timestamp_millis, WEEK_IN_MS};
pub use gldt_stake_api_canister::get_apy_timeseries::{
    Args as GetApyTimeseriesArgs, Response as GetApyTimeseriesResponse,
};
use ic_cdk::query;
use ic_stable_structures::BTreeMap;
use tracing::info;
use types::TimestampMillis;

use crate::{memory::VM, state::read_state};

#[query]
fn get_apy_timeseries(args: GetApyTimeseriesArgs) -> GetApyTimeseriesResponse {
    let weekly_apy_history: Vec<(TimestampMillis, f64)> =
        read_state(|s| s.data.stake_system.weekly_apy_history.iter().collect());
    let limit = args.limit.unwrap_or(usize::MAX);
    let starting_week = args.starting_week.clone();
    get_weekly_series(starting_week, limit, &weekly_apy_history)
}

fn get_weekly_series(
    starting_week: u64,
    limit: usize,
    weekly_apy_history: &Vec<(TimestampMillis, f64)>,
) -> Vec<(TimestampMillis, f64)> {
    weekly_apy_history
        .into_iter()
        .rev()
        .enumerate()
        .filter(|(week, _)| *week as u64 >= starting_week)
        .map(|(_, data)| data)
        .take(limit)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use candid::Nat;
    use canister_time::{timestamp_millis, WEEK_IN_MS};
    use time::{Duration, OffsetDateTime};
    use types::TimestampMillis;

    use super::get_weekly_series;

    #[test]
    fn test_get_weekly_series_paginated() {
        // --------------------------------------
        //        basic happy path test
        // --------------------------------------
        let mut weekly_history: Vec<(TimestampMillis, f64)> = vec![];
        let timestamp = timestamp_millis();

        for i in 0..250u64 {
            weekly_history.push((timestamp + WEEK_IN_MS, i as f64))
        }

        let res = get_weekly_series(0, 250, &weekly_history);
        assert_eq!(res.len(), 250);

        let res = get_weekly_series(0, 100, &weekly_history);
        assert_eq!(res.len(), 100);
        assert_eq!(res[0].1, 249.0);
        assert_eq!(res[99].1, 150.0);

        let res = get_weekly_series(100, 100, &weekly_history);
        assert_eq!(res.len(), 100);
        assert_eq!(res[0].1, 149.0);
        assert_eq!(res[99].1, 50.0);

        let res = get_weekly_series(200, 100, &weekly_history);
        assert_eq!(res.len(), 50);
        assert_eq!(res[0].1, 49.0);
        assert_eq!(res.last().unwrap().1, 0.0);
    }
}
