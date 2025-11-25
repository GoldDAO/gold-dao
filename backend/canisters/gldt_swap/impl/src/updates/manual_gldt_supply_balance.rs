#![allow(unused_imports)]

use crate::guards::caller_is_authorized;
use crate::state::mutate_state;
pub use gldt_swap_api_canister::manual_gldt_supply_balance::{
    Args as ManualGldtSupplyBalanceArgs, Response as ManualGldtSupplyBalanceResponse,
};
use ic_cdk::update;

#[update(hidden = true, guard = "caller_is_authorized")]
async fn manual_gldt_supply_balance(
    _: ManualGldtSupplyBalanceArgs,
) -> ManualGldtSupplyBalanceResponse {
    manual_gldt_supply_balance_impl().await
}

async fn manual_gldt_supply_balance_impl() {
    mutate_state(|s| {
        s.data.is_gldt_supply_balancer_running = false;
    });

    crate::jobs::manage_token_supply::spawn_gldt_supply_balancer();
}
