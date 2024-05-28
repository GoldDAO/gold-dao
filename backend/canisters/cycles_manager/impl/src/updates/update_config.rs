use crate::guards::caller_is_governance_principal;
use crate::state::{mutate_state, State};
use canister_tracing_macros::trace;
pub use cycles_manager_canister::update_config::Args as UpdateConfigArgs;
pub use cycles_manager_canister::update_config::Response as UpdateConfigResponse;
use ic_cdk_macros::update;

#[update(guard = "caller_is_governance_principal")]
#[trace]
fn update_config(args: UpdateConfigArgs) -> UpdateConfigResponse {
    mutate_state(|state| update_config_impl(args, state))
}

fn update_config_impl(args: UpdateConfigArgs, state: &mut State) -> UpdateConfigResponse {
    if let Some(max_top_up_amount) = args.max_top_up_amount {
        state.data.max_top_up_amount = max_top_up_amount;
    }
    if let Some(min_interval) = args.min_interval {
        state.data.min_interval = min_interval;
    }
    if let Some(min_cycles_balance) = args.min_cycles_balance {
        state.data.min_cycles_balance = min_cycles_balance;
    }
    UpdateConfigResponse::Success
}
