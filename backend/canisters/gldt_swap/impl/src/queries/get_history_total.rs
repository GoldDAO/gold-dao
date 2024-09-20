use candid::Nat;
pub use gldt_swap_api_canister::get_history_total::{
    Args as GetHistoryTotalArgs,
    Response as GetHistoryTotalResponse,
};
use ic_cdk::query;
use tracing::debug;

use crate::{ state::read_state, utils::get_all_user_swap_ids };

#[query(composite = true)]
async fn get_history_total(user: GetHistoryTotalArgs) -> GetHistoryTotalResponse {
    match user {
        Some(principal) => {
            // archives largest index to smallest index
            match get_all_user_swap_ids(&principal).await {
                Ok(all_user_swaps) => Nat::from(all_user_swaps.len()),
                Err(e) => {
                    debug!("ERROR : get_history_total : user total : {e:?}");
                    return Nat::from(0u64);
                }
            }
        }
        None => { read_state(|s| s.data.swaps.get_history_total()) }
    }
}
