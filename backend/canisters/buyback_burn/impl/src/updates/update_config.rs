use crate::guards::caller_is_governance_principal;
use crate::state::{mutate_state, State};
pub use buyback_burn_canister::update_config::Args as UpdateConfigArgs;
pub use buyback_burn_canister::update_config::Response as UpdateConfigResponse;
use canister_tracing_macros::trace;
use ic_cdk_macros::update;

#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
fn update_config(args: UpdateConfigArgs) -> UpdateConfigResponse {
    mutate_state(|state| update_config_impl(args, state))
}

fn update_config_impl(args: UpdateConfigArgs, state: &mut State) -> UpdateConfigResponse {
    if let Some(min_burn_amount) = args.min_burn_amount {
        state.data.proposal_config.min_burn_amount = min_burn_amount;
    }
    UpdateConfigResponse::Success
}
