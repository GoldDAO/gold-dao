use ic_cdk::query;
use crate::core::working_stats::api_count;
pub use super_stats_v3_api::{
    runtime::RUNTIME_STATE,
    stable_memory::STABLE_STATE,
    stats::queries::get_total_holders::Response as GetTotalHoldersResponse,
};

#[query]
pub fn get_total_holders() -> GetTotalHoldersResponse {
    // check authorised
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text()) });

    // account count
    let ac_len = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().account_data.accounts.len()
    });

    // principal count
    let pr_len = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().principal_data.accounts.len()
    });
    api_count();
    return GetTotalHoldersResponse {
        total_accounts: ac_len,
        total_principals: pr_len,
    };
}
