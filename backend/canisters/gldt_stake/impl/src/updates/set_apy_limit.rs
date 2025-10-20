use crate::guards::caller_is_governance_principal;
use crate::state::mutate_state;
use crate::state::RuntimeState;
use canister_tracing_macros::trace;
pub use gldt_stake_api_canister::set_apy_limit::{
    Args as SetAPYLimitArgs, Response as SetAPYLimitResponse,
};
use ic_cdk::query;
use ic_cdk::update;
use utils::numeric::Percentage;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn set_apy_limit_validate(args: SetAPYLimitArgs) -> Result<String, String> {
    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal")]
#[trace]
fn set_apy_limit(args: SetAPYLimitArgs) -> SetAPYLimitResponse {
    mutate_state(|s| set_apy_limit_impl(args, s))
}

fn set_apy_limit_impl(args: SetAPYLimitArgs, state: &mut RuntimeState) -> SetAPYLimitResponse {
    match args {
        Some(apy_limit) => {
            match Percentage::new(apy_limit) {
                Ok(percentage) => state.data.stake_system.apy_limit = Some(percentage),
                Err(_) => return Err("Invalid APY limit percentage".to_string()),
            };
        }
        None => {
            state.data.stake_system.apy_limit = None;
        }
    }

    Ok(())
}
