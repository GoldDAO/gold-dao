pub use gldt_swap_api_canister::get_active_swap_ids_by_user::{
    Args as GetActiveSwapIndexsByUserArgs, Response as GetActiveSwapIndexsByUserResponse,
};
use ic_cdk::query;
use utils::env::Environment;

use crate::state::read_state;

#[query]
async fn get_active_swap_ids_by_user(
    args: GetActiveSwapIndexsByUserArgs,
) -> GetActiveSwapIndexsByUserResponse {
    let user_principal = args.unwrap_or(read_state(|s| s.env.caller()));
    read_state(|s| {
        s.data
            .swap_system
            .get_active_swaps_by_user_principal(user_principal)
            .iter()
            .map(|(swap_id, _)| swap_id.clone())
            .collect()
    })
}
