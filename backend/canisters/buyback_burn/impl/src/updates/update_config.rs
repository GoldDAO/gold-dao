use crate::guards::caller_is_governance_principal;
use crate::state::{ mutate_state, RuntimeState };
pub use buyback_burn_canister::update_config::Args as UpdateConfigArgs;
pub use buyback_burn_canister::update_config::Response as UpdateConfigResponse;
use canister_tracing_macros::trace;
use ic_cdk_macros::update;
use std::time::Duration;

#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
fn update_config(args: UpdateConfigArgs) -> UpdateConfigResponse {
    mutate_state(|state| update_config_impl(args, state))
}

fn update_config_impl(args: UpdateConfigArgs, state: &mut RuntimeState) -> UpdateConfigResponse {
    if let Some(burn_rate) = args.burn_rate {
        state.data.burn_config.burn_rate = burn_rate;
    }
    if let Some(min_icp_burn_amount) = args.min_icp_burn_amount {
        state.data.burn_config.min_icp_burn_amount = min_icp_burn_amount;
    }
    if let Some(burn_interval) = args.burn_interval_in_secs {
        state.data.burn_config.burn_interval = Duration::from_secs(burn_interval);
    }

    UpdateConfigResponse::Success
}
