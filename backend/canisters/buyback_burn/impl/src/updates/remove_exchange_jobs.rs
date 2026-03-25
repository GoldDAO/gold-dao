use crate::guards::caller_is_governance_principal;
use crate::state::{mutate_state, RuntimeState};
use bity_ic_canister_tracing_macros::trace;
pub use buyback_burn_api::remove_exchange_jobs::Args as RemoveExchangeJobArgs;
pub use buyback_burn_api::remove_exchange_jobs::Response as RemoveExchangeJobResponse;
use ic_cdk_macros::{query, update};

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn remove_exchange_jobs_validate(args: RemoveExchangeJobArgs) -> Result<String, String> {
    args.validate()
        .map_err(|e| format!("Validation error: {}", e))?;

    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
fn remove_exchange_jobs(args: RemoveExchangeJobArgs) -> RemoveExchangeJobResponse {
    mutate_state(|state| remove_exchange_jobs_impl(args, state))
}

fn remove_exchange_jobs_impl(
    args: RemoveExchangeJobArgs,
    state: &mut RuntimeState,
) -> RemoveExchangeJobResponse {
    for exchange_job_config in args.exchange_configs {
        let _ = state
            .data
            .exchange_jobs
            .remove_exchange_job(exchange_job_config);
    }

    RemoveExchangeJobResponse::Success
}
