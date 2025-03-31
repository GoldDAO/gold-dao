use crate::guards::caller_is_governance_principal;
use crate::state::{mutate_state, RuntimeState};
use canister_tracing_macros::trace;
use ic_cdk_macros::{query, update};
pub use sns_neuron_controller_api_canister::update_ogy_config::Args as UpdateOgyConfigArgs;
pub use sns_neuron_controller_api_canister::update_ogy_config::Response as UpdateOgyConfigResponse;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn update_ogy_config_validate(args: UpdateOgyConfigArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
fn update_ogy_config(args: UpdateOgyConfigArgs) -> UpdateOgyConfigResponse {
    mutate_state(|state| update_config_impl(args, state))
}

fn update_config_impl(
    args: UpdateOgyConfigArgs,
    state: &mut RuntimeState,
) -> UpdateOgyConfigResponse {
    if let Some(ogy_sns_governance_canister_id) = args.ogy_sns_governance_canister_id {
        state
            .data
            .neuron_managers
            .ogy
            .ogy_sns_governance_canister_id = ogy_sns_governance_canister_id;
    }

    if let Some(ogy_sns_ledger_canister_id) = args.ogy_sns_ledger_canister_id {
        state.data.neuron_managers.ogy.ogy_sns_ledger_canister_id = ogy_sns_ledger_canister_id;
    }

    if let Some(ogy_sns_rewards_canister_id) = args.ogy_sns_rewards_canister_id {
        state.data.neuron_managers.ogy.ogy_sns_rewards_canister_id = ogy_sns_rewards_canister_id;
    }

    if let Some(ogy_rewards_threshold) = args.ogy_rewards_threshold {
        state.data.neuron_managers.ogy.ogy_rewards_threshold = ogy_rewards_threshold;
    }

    UpdateOgyConfigResponse::Success
}
