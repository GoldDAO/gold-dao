use crate::guards::caller_is_governance_principal;
use crate::state::{mutate_state, RuntimeState};
use bity_ic_canister_tracing_macros::trace;
pub use buyback_burn_api::update_exchange_config::Args as UpdateExchangeConfigArgs;
pub use buyback_burn_api::update_exchange_config::Response as UpdateExchangeConfigResponse;
use ic_cdk_macros::{query, update};

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn update_exchange_config_validate(args: UpdateExchangeConfigArgs) -> Result<String, String> {
    args.validate()
        .map_err(|e| format!("Validation error: {}", e))?;

    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
fn update_exchange_config(args: UpdateExchangeConfigArgs) -> UpdateExchangeConfigResponse {
    mutate_state(|state| update_exchange_config_impl(args, state))
}

fn update_exchange_config_impl(
    args: UpdateExchangeConfigArgs,
    state: &mut RuntimeState,
) -> UpdateExchangeConfigResponse {
    let _ = state
        .data
        .exchange_jobs
        .update_config_and_restart_timer(args);
    UpdateExchangeConfigResponse::Success
}
