use crate::guards::caller_is_governance_principal;
use crate::state::{ mutate_state, RuntimeState };
pub use buyback_burn_api::update_config::Args as UpdateConfigArgs;
pub use buyback_burn_api::update_config::Response as UpdateConfigResponse;
use canister_tracing_macros::trace;
use ic_cdk_macros::{ update, query };
use std::time::Duration;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn update_config_validate(args: UpdateConfigArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
fn update_config(args: UpdateConfigArgs) -> UpdateConfigResponse {
    mutate_state(|state| update_config_impl(args, state))
}

fn update_config_impl(args: UpdateConfigArgs, state: &mut RuntimeState) -> UpdateConfigResponse {
    if let Some(burn_rate) = args.burn_rate {
        if burn_rate > 100 {
            return UpdateConfigResponse::InvalidBurnRate;
        } else {
            state.data.burn_config.burn_rate = burn_rate;
        }
    }
    if let Some(min_burn_amount) = args.min_burn_amount {
        state.data.burn_config.min_burn_amount = min_burn_amount;
    }
    if let Some(buyback_burn_interval_in_secs) = args.buyback_burn_interval_in_secs {
        state.data.buyback_burn_interval = Duration::from_secs(buyback_burn_interval_in_secs);
    }

    UpdateConfigResponse::Success
}
