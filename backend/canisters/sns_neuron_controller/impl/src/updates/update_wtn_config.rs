use crate::guards::caller_is_governance_principal;
use crate::state::{mutate_state, RuntimeState};
use canister_tracing_macros::trace;
use ic_cdk_macros::{query, update};
pub use sns_neuron_controller_api_canister::update_wtn_config::Args as UpdateWtnConfigArgs;
pub use sns_neuron_controller_api_canister::update_wtn_config::Response as UpdateWtnConfigResponse;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn update_wtn_config_validate(args: UpdateWtnConfigArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
fn update_wtn_config(args: UpdateWtnConfigArgs) -> UpdateWtnConfigResponse {
    mutate_state(|state| update_config_impl(args, state))
}

fn update_config_impl(
    args: UpdateWtnConfigArgs,
    state: &mut RuntimeState,
) -> UpdateWtnConfigResponse {
    if let Some(wtn_sns_governance_canister_id) = args.wtn_sns_governance_canister_id {
        state
            .data
            .neuron_managers
            .wtn
            .wtn_sns_governance_canister_id = wtn_sns_governance_canister_id;
    }

    if let Some(wtn_sns_ledger_canister_id) = args.wtn_sns_ledger_canister_id {
        state.data.neuron_managers.wtn.wtn_sns_ledger_canister_id = wtn_sns_ledger_canister_id;
    }

    if let Some(icp_ledger) = args.icp_ledger {
        state.data.neuron_managers.wtn.icp_ledger = icp_ledger;
    }

    if let Some(icp_rewards_threshold) = args.icp_rewards_threshold {
        state.data.neuron_managers.wtn.icp_rewards_threshold = icp_rewards_threshold;
    }

    if let Some(wtn_rewards_threshold) = args.wtn_rewards_threshold {
        state.data.neuron_managers.wtn.wtn_rewards_threshold = wtn_rewards_threshold;
    }

    UpdateWtnConfigResponse::Success
}
