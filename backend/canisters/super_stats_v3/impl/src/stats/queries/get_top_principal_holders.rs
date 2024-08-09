use ic_cdk::query;
use crate::{ core::working_stats::api_count, stats::directory::lookup_directory };
pub use super_stats_v3_api::{
    custom_types::{ HolderBalance, HolderBalanceResponse },
    runtime::RUNTIME_STATE,
    stable_memory::STABLE_STATE,
    stats::queries::get_top_principal_holders::Response as GetTopPrincipalHoldersResponse,
};

#[query]
pub fn get_top_principal_holders(number_to_return: u64) -> GetTopPrincipalHoldersResponse {
    // check authorised
    RUNTIME_STATE.with(|s| { s.borrow().data.check_authorised(ic_cdk::caller().to_text()) });

    let top: Vec<HolderBalance> = STABLE_STATE.with(|s| {
        let mut ac_vec: Vec<HolderBalance> = Vec::new();
        for ac in s.borrow().as_ref().unwrap().principal_data.accounts.iter() {
            let refs = ac.0.clone();
            let ov = ac.1.clone();
            ac_vec.push(HolderBalance {
                holder: refs,
                data: ov,
            });
        }

        // catch 0 result
        if ac_vec.len() == 0 {
            return ac_vec;
        }

        ac_vec.sort_unstable_by_key(|element| element.data.balance);
        ac_vec.reverse();
        let mut top_ac: Vec<HolderBalance> = Vec::new();
        for i in 0..number_to_return as usize {
            top_ac.push(ac_vec[i].to_owned());
        }
        api_count();
        return top_ac;
    });

    // replace ref with full accounts
    let mut return_data: Vec<HolderBalanceResponse> = Vec::new();
    for ad in top {
        let ac = lookup_directory(ad.holder);
        match ac {
            Some(v) => {
                return_data.push(HolderBalanceResponse {
                    holder: v,
                    data: ad.data,
                });
            }
            None => {} // do nothing
        }
    }
    api_count();
    return return_data;
}
