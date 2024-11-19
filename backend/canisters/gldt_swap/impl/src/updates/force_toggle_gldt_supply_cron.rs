#![allow(dead_code)] // Ignore warnings for unused code (functions, structs, etc.)
#![allow(unused_imports)] // Ignore warnings for unused imports
#![allow(unused_variables)] // Ignore warnings for unused variables
#![allow(unused_mut)] // Ignore warnings for unused mutable variables
#![allow(unused_macros)]

use crate::guards::caller_is_authorized;
use crate::state::mutate_state;
use crate::swap::swap_info::SwapInfoTrait;
pub use gldt_swap_api_canister::force_toggle_gldt_supply_cron::{
    Args as ForceSetGldtSupplyCronArgs, Response as ForceSetGldtSupplyCronResponse,
};
use ic_cdk::update;

#[cfg(feature = "inttest")]
#[update(hidden = true, guard = "caller_is_authorized")]
async fn force_toggle_gldt_supply_cron(
    collection_id: ForceSetGldtSupplyCronArgs,
) -> ForceSetGldtSupplyCronResponse {
    force_toggle_gldt_supply_cron_impl(collection_id).await
}

async fn force_toggle_gldt_supply_cron_impl(collection_id: ForceSetGldtSupplyCronArgs) {
    // valid and create new swaps - error swaps are saved too
    mutate_state(|s| {
        s.data.is_gldt_supply_balancer_running = !s.data.is_gldt_supply_balancer_running;
    })
}
