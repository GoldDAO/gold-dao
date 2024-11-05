pub use gldt_swap_api_canister::get_swap::{Args as GetSwapArgs, Response as GetSwapResponse};
use ic_cdk::update;

use crate::{state::read_state, utils::get_historic_swap};

#[update]
async fn get_swap(swap_id: GetSwapArgs) -> GetSwapResponse {
    let active_swap = read_state(|s| s.data.swaps.get_active_swap(&swap_id).cloned());
    let historic_swap = get_historic_swap(&swap_id).await;

    let swap = active_swap.or(historic_swap);

    match swap {
        Some(s) => Some((swap_id.clone(), s.clone())),
        None => None,
    }
}
