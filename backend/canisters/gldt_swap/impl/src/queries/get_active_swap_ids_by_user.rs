pub use gldt_swap_api_canister::get_active_swap_ids_by_user::{
    Args as GetActiveSwapIdsByUserArgs, Response as GetActiveSwapIdsByUserResponse,
};
use ic_cdk::query;
use utils::env::Environment;

use crate::state::read_state;

#[query]
async fn get_active_swap_ids_by_user(
    args: GetActiveSwapIdsByUserArgs,
) -> GetActiveSwapIdsByUserResponse {
    let user_principal = args.unwrap_or(read_state(|s| s.env.caller()));
    read_state(|s| {
        s.data
            .swaps
            .get_active_swaps_by_user_principal(user_principal)
            .iter()
            .map(|(swap_id, _)| swap_id.clone())
            .collect()
    })
}
