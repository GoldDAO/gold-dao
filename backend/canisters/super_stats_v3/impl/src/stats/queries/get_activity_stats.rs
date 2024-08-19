use ic_cdk::query;
use crate::core::working_stats::api_count;
pub use super_stats_v3_api::{
    runtime::RUNTIME_STATE,
    stable_memory::STABLE_STATE,
    stats::queries::get_activity_stats::{
        Args as GetActivityStatsArgs,
        Response as GetActivityStatsResponse,
    },
};

#[query]
pub fn get_activity_stats(days: GetActivityStatsArgs) -> GetActivityStatsResponse {
    // Check authorization
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text()) });
    api_count();

    let mut snapshots = STABLE_STATE.with(|s| {
        s.borrow()
            .as_ref()
            .unwrap()
            .activity_stats.get_daily_snapshots(days as usize)
    });

    let ac_len = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().account_data.accounts.len()
    });

    let pr_len = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().principal_data.accounts.len()
    });

    // Update the last snapshot with the live account and principal count
    if let Some(last_snapshot) = snapshots.last_mut() {
        last_snapshot.total_unique_accounts = ac_len as u64;
        last_snapshot.total_unique_principals = pr_len as u64;
    }

    snapshots
}
