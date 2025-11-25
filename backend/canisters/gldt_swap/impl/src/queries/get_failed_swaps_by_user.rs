pub use gldt_swap_api_canister::get_failed_swaps_by_user::{
    Args as GetFailedSwapsByUserArgs, Response as GetFailedSwapsByUserResponse,
};
use ic_cdk::query;
use utils::env::Environment;

use crate::state::read_state;

#[query]
async fn get_failed_swaps_by_user(args: GetFailedSwapsByUserArgs) -> GetFailedSwapsByUserResponse {
    let user_principal = args.unwrap_or(read_state(|s| s.env.caller()));
    read_state(|s| {
        s.data
            .swap_system
            .get_failed_swaps_by_user_principal(user_principal)
    })
}
