use crate::guards::caller_is_authorized;
use crate::state::mutate_state;
pub use gldt_swap_api_canister::manual_toggle_supply_balancer_var::{
    Args as ForceSetGldtSupplyCronArgs, Response as ForceSetGldtSupplyCronResponse,
};
use ic_cdk::update;

#[update(hidden = true, guard = "caller_is_authorized")]
async fn manual_toggle_supply_balancer_var(
    _: ForceSetGldtSupplyCronArgs,
) -> ForceSetGldtSupplyCronResponse {
    manual_toggle_supply_balancer_var_impl().await
}

async fn manual_toggle_supply_balancer_var_impl() {
    mutate_state(|s| {
        s.data.is_gldt_supply_balancer_running = !s.data.is_gldt_supply_balancer_running;
    })
}
