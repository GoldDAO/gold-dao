use crate::guards::caller_is_governance_principal;
use crate::state::mutate_state;
use bity_ic_canister_tracing_macros::trace;
pub use buyback_burn_api::update_stake_icp_config::Args as UpdateStakeIcpConfigArgs;
pub use buyback_burn_api::update_stake_icp_config::Response as UpdateStakeIcpConfigResponse;
use ic_cdk_macros::{query, update};

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn update_stake_icp_config_validate(
    args: UpdateStakeIcpConfigArgs,
) -> Result<String, String> {
    if let Some(config) = &args.config {
        config
            .validate()
            .map_err(|e| format!("Validation error: {}", e))?;
    }
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
fn update_stake_icp_config(args: UpdateStakeIcpConfigArgs) -> UpdateStakeIcpConfigResponse {
    if let Some(config) = &args.config {
        if let Err(e) = config.validate() {
            return UpdateStakeIcpConfigResponse::ValidationError(e);
        }
    }

    mutate_state(|state| {
        state.data.stake_icp_config = args.config;
    });

    UpdateStakeIcpConfigResponse::Success
}
