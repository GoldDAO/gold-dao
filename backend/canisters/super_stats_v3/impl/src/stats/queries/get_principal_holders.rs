use ic_cdk::query;
use crate::{ core::working_stats::api_count, stats::directory::lookup_directory };
pub use super_stats_v3_api::{
    custom_types::{ HolderBalance, HolderBalanceResponse },
    runtime::RUNTIME_STATE,
    stable_memory::STABLE_STATE,
    stats::queries::get_principal_holders::{
        Args as GetPrincipalHoldersArgs,
        Response as GetPrincipalHoldersResponse,
    },
};

#[query]
pub fn get_principal_holders(args: GetPrincipalHoldersArgs) -> GetPrincipalHoldersResponse {
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
        if ac_vec.is_empty() {
            return Vec::new();
        }

        ac_vec.sort_unstable_by_key(|element| element.data.balance);
        ac_vec.reverse();

        let start_index = args.offset as usize;
        let end_index = (args.offset + args.limit) as usize;

        // Ensure end_index doesn't exceed the vector length
        let end_index = if end_index > ac_vec.len() { ac_vec.len() } else { end_index };

        ac_vec[start_index..end_index].to_vec()
    });

    // replace ref with full accounts
    let mut return_data: Vec<HolderBalanceResponse> = Vec::new();
    for ad in top {
        let ac = lookup_directory(ad.holder);
        if let Some(v) = ac {
            return_data.push(HolderBalanceResponse {
                holder: v,
                data: ad.data,
            });
        }
    }
    api_count();
    return return_data;
}
