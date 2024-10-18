use crate::guards::caller_is_governance_principal;
use crate::state::{ mutate_state, RuntimeState };
pub use buyback_burn_api::add_swap_client::Args as AddSwapClientArgs;
pub use buyback_burn_api::add_swap_client::Response as AddSwapClientResponse;
use canister_tracing_macros::trace;
use ic_cdk_macros::{ update, query };

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn add_swap_clients_validate(args: AddSwapClientArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
fn add_swap_clients(args: AddSwapClientArgs) -> AddSwapClientResponse {
    mutate_state(|state| update_config_impl(args, state))
}

fn update_config_impl(args: AddSwapClientArgs, state: &mut RuntimeState) -> AddSwapClientResponse {
    for token in args.tokens.iter() {
        state.data.swap_clients.add_swap_client(
            token.token,
            state.data.gldgov_token_info,
            token.swap_pool_id
        );
    }

    AddSwapClientResponse::Success
}
