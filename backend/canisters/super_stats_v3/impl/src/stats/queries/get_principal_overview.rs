use ic_cdk::query;
use crate::core::working_stats::api_count;
pub use super_stats_v3_api::{
    runtime::RUNTIME_STATE,
    stable_memory::STABLE_STATE,
    stats::queries::get_principal_overview::Response as GetPrincipalOverviewResponse,
};

#[query]
pub fn get_principal_overview(account: String) -> GetPrincipalOverviewResponse {
    // check authorised
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text()) });
    api_count();
    // get ac_ref
    let ac_ref = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().directory_data.get_ref(&account)
    });
    match ac_ref {
        Some(v) => {
            let ret = STABLE_STATE.with(|s| {
                match s.borrow().as_ref().unwrap().principal_data.accounts.get(&v) {
                    Some(v) => {
                        let ov = v.to_owned();
                        return Some(ov);
                    }
                    None => {
                        return None;
                    }
                }
            });
            return ret;
        }
        None => {
            return None;
        }
    }
}
