use ic_cdk::query;
use utils::time::fill_missing_days;
use crate::{ core::working_stats::api_count, stats::history::get_history_of_account };
pub use super_stats_v3_api::{
    account_tree::HistoryData,
    runtime::RUNTIME_STATE,
    stats::queries::get_principal_history::{
        Args as GetPrincipalHistoryArgs,
        Response as GetPrincipalHistoryResponse,
    },
};

#[query]
pub fn get_principal_history(args: GetPrincipalHistoryArgs) -> GetPrincipalHistoryResponse {
    // check authorised
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text()) });
    api_count();
    let history = get_history_of_account(args.account, args.days, true);
    let default_history_data = HistoryData {
        balance: 0,
    };
    let mut filled_history = fill_missing_days(history, args.days, default_history_data);
    let ret = filled_history.split_off(filled_history.len() - (args.days as usize));
    return ret;
}
