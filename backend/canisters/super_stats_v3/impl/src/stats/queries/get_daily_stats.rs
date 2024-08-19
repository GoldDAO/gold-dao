use ic_cdk::query;
use crate::core::working_stats::api_count;
pub use super_stats_v3_api::{
    runtime::RUNTIME_STATE,
    stats::queries::get_daily_stats::Response as GetDailyStatsResponse,
};

#[query]
pub fn get_daily_stats() -> GetDailyStatsResponse {
    // check authorised
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text()) });
    api_count();
    RUNTIME_STATE.with(|s| { s.borrow().data.daily_stats.clone() })
}
