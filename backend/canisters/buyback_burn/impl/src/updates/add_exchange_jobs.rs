use crate::guards::caller_is_governance_principal;
use crate::state::{mutate_state, RuntimeState};
use bity_ic_canister_tracing_macros::trace;
pub use buyback_burn_api::add_exchange_jobs::Args as AddExchangeJobArgs;
pub use buyback_burn_api::add_exchange_jobs::Response as AddExchangeJobResponse;
use ic_cdk_macros::{query, update};
use tracing::error;

#[query(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
async fn add_exchange_jobs_validate(args: AddExchangeJobArgs) -> Result<String, String> {
    for config in &args.exchange_configs {
        config.validate()?;
    }

    serde_json::to_string_pretty(&args).map_err(|_| "invalid payload".to_string())
}

#[update(guard = "caller_is_governance_principal", hidden = true)]
#[trace]
fn add_exchange_jobs(args: AddExchangeJobArgs) -> AddExchangeJobResponse {
    mutate_state(|state| add_exchange_jobs_impl(args, state))
}

fn add_exchange_jobs_impl(
    args: AddExchangeJobArgs,
    state: &mut RuntimeState,
) -> AddExchangeJobResponse {
    for exchange_job_config in args.exchange_configs {
        match exchange_job_config.validate() {
            Ok(_) => {
                let _ = state
                    .data
                    .exchange_jobs
                    .add_exchange_job(exchange_job_config);
            }
            Err(e) => {
                error!("Invalid exchange job config provided during init: {}", e);
            }
        }
    }

    AddExchangeJobResponse::Success
}
