use ic_cdk::query;
use utils::principal::validate_principal_dot_account;
use crate::core::working_stats::api_count;
pub use super_stats_v3_api::{
    runtime::RUNTIME_STATE,
    stable_memory::STABLE_STATE,
    stats::queries::get_account_overview::Response as GetAccountOverviewResponse,
};

#[query]
pub fn get_account_overview(account: String) -> GetAccountOverviewResponse {
    // check authorised
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text()) });
    api_count();

    match validate_principal_dot_account(&account.as_str()) {
        Some(valid_account_input) => {
            // get ac_ref
            let ac_ref = STABLE_STATE.with(|s| {
                s.borrow().as_ref().unwrap().directory_data.get_ref(&valid_account_input)
            });
            match ac_ref {
                Some(v) => {
                    let ret = STABLE_STATE.with(|s| {
                        match s.borrow().as_ref().unwrap().account_data.accounts.get(&v) {
                            Some(v) => {
                                let ov = v.to_owned();
                                return Some(ov);
                            }
                            None => None,
                        }
                    });
                    return ret;
                }
                None => None,
            }
        }
        None => None,
    }
}
