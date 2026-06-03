use self::types::state::{ExchangeJobsV0, RuntimeStateV0};
use crate::state::Data;
use crate::state::RuntimeState;
use crate::types::exchange_jobs::{ExchangeJob, ExchangeJobs};
use buyback_burn_api::stake_icp_config::StakeIcpConfig;

pub mod types;

impl From<ExchangeJobsV0> for ExchangeJobs {
    fn from(old: ExchangeJobsV0) -> Self {
        let exchange_jobs = old
            .exchange_jobs
            .into_iter()
            .map(|(id, job)| {
                (
                    id,
                    ExchangeJob {
                        id: job.id,
                        exchange: job.exchange,
                        rate_per_interval: job.rate_per_interval,
                        timer_id: None,
                        job_interval: job.job_interval,
                        source_subaccount: job.source_subaccount,
                        min_amount: job.min_amount,
                        max_amount: job.max_amount,
                        destination_account: job.destination_account,
                        constraints: vec![],
                        post_transfer_action: None,
                    },
                )
            })
            .collect();
        ExchangeJobs {
            exchange_jobs,
            last_used_id: old.last_used_id,
        }
    }
}

impl From<RuntimeStateV0> for RuntimeState {
    fn from(old_state: RuntimeStateV0) -> Self {
        Self {
            env: old_state.env,
            data: Data {
                authorized_principals: old_state.data.authorized_principals,
                icp_swap_canister_id: old_state.data.icp_swap_canister_id,
                exchange_jobs: old_state.data.exchange_jobs.into(),
                exchange_job_guards: Default::default(),
                token_swaps: old_state.data.token_swaps.into(),
                stake_icp_config: Some(StakeIcpConfig::default()),
            },
        }
    }
}
