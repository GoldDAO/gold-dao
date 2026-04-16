use crate::types::SwapClientEnum;
use crate::utils::run_now_then_interval_with_args;
use crate::UpdateExchangeConfigArgs;
use buyback_burn_api::exchange_job_config::ExchangeJobConfig;
use buyback_burn_api::swap_config::SwapConfig;
use ic_cdk_timers::TimerId;
use ic_ledger_types::Tokens;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::info;
use utils::numeric::Rate;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ExchangeJobs {
    pub exchange_jobs: HashMap<u128, ExchangeJob>,
    pub last_used_id: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExchangeJob {
    pub id: u128,
    pub exchange: SwapClientEnum,
    pub rate_per_interval: Rate,
    // NOTE: timers are not preserved, so there is no need to serialize them
    #[serde(skip)]
    pub timer_id: Option<TimerId>,
    pub job_interval: Duration,
    pub source_subaccount: Option<Subaccount>, // NOTE: subaccount from which tokens are sold
    pub min_amount: Tokens,                    // in token that is bought
    pub max_amount: Option<Tokens>,
    pub destination_account: Option<Account>,
}

impl TryFrom<(u128, ExchangeJobConfig)> for ExchangeJob {
    type Error = String;

    fn try_from((exchange_job_id, config): (u128, ExchangeJobConfig)) -> Result<Self, Self::Error> {
        Ok(Self {
            id: exchange_job_id,
            exchange: SwapClientEnum::build_swap_client(SwapConfig {
                swap_client_id: exchange_job_id,
                input_token: config.token_to_sell,
                output_token: config.token_to_buy,
                exchange_config: config.exchange,
            }),
            rate_per_interval: Rate::from_scaled(config.rate_per_interval),
            timer_id: None,
            job_interval: Duration::from_millis(config.job_interval_ms),
            source_subaccount: config.source_subaccount,
            min_amount: config.min_amount,
            max_amount: config.max_amount,
            destination_account: config.destination_account,
        })
    }
}

impl ExchangeJobs {
    pub fn init() -> Self {
        Self {
            exchange_jobs: HashMap::new(),
            last_used_id: 0,
        }
    }

    // pub fn add_exchange_job(
    //     &mut self,
    //     exchange_job_config: ExchangeJobConfig,
    // ) -> Result<(), String> {
    //     let exchange_job_id = self.get_next_id();
    //     let exchange_job: ExchangeJob = (exchange_job_id, exchange_job_config).try_into()?;

    //     self.exchange_jobs
    //         .insert(exchange_job_id, exchange_job.clone());

    //     let timer_id = run_now_then_interval_with_args(exchange_job.job_interval, move || {
    //         crate::jobs::swap_tokens::run(exchange_job_id);
    //     });

    //     let job = self
    //         .exchange_jobs
    //         .get_mut(&exchange_job_id)
    //         .ok_or(String::from("Job not found"))?;
    //     job.timer_id = Some(timer_id);

    //     self.last_used_id = exchange_job_id;
    //     info!("Added exchange job {}", exchange_job_id);
    //     Ok(())
    // }

    pub fn add_exchange_job(&mut self, config: ExchangeJobConfig) -> Result<(), String> {
        let id = self.get_next_id();
        let mut job: ExchangeJob = (id, config).try_into()?;

        // 1. Insert into state FIRST so 'run(id)' can find it
        self.exchange_jobs.insert(id, job.clone());
        self.last_used_id = id;

        // 2. Start the timer
        let timer_id = run_now_then_interval_with_args(job.job_interval, move || {
            crate::jobs::swap_tokens::run(id);
        });

        // 3. Update the stored job with the timer_id
        if let Some(stored_job) = self.exchange_jobs.get_mut(&id) {
            stored_job.timer_id = Some(timer_id);
        }

        info!("Added exchange job {}", id);
        Ok(())
    }

    pub fn remove_exchange_job(&mut self, exchange_job_id: u128) -> Result<(), String> {
        let job_opt = self.exchange_jobs.remove(&exchange_job_id);

        if let Some(job) = job_opt {
            if let Some(timer_id) = job.timer_id {
                ic_cdk_timers::clear_timer(timer_id);
            }
            info!("Cleared timer for exchange job {}", exchange_job_id);
        }

        info!("Removed exchange job {}", exchange_job_id);
        Ok(())
    }

    pub fn clear_exchange_jobs(&mut self) {
        self.exchange_jobs.clear();
        self.last_used_id = 0;
    }

    pub fn get_next_id(&self) -> u128 {
        self.last_used_id + 1
    }

    pub fn get_exchange_job(&self, swap_client_id: u128) -> Option<&ExchangeJob> {
        self.exchange_jobs.get(&swap_client_id)
    }

    pub fn iter(&self) -> std::collections::hash_map::Values<'_, u128, ExchangeJob> {
        self.exchange_jobs.values()
    }

    pub fn update_config_and_restart_timer(
        &mut self,
        update: UpdateExchangeConfigArgs,
    ) -> Result<(), String> {
        // Get the updated job
        let job = self
            .exchange_jobs
            .get_mut(&update.exchange_job_id)
            .ok_or(String::from("Job not found"))?;

        if let Some(rate_per_interval) = update.rate_per_interval {
            job.rate_per_interval = Rate::from_scaled(rate_per_interval);
        }

        if let Some(source_subaccount) = update.source_subaccount {
            job.source_subaccount = source_subaccount;
        }

        if let Some(min_amount) = update.min_amount {
            job.min_amount = min_amount;
        }

        if let Some(max_amount) = update.max_amount {
            job.max_amount = max_amount;
        }

        if let Some(destination_account) = update.destination_account {
            job.destination_account = destination_account;
        }

        // Validate: max_amount must be greater than min_amount if both exist
        if let Some(max) = job.max_amount {
            if max.e8s() <= job.min_amount.e8s() {
                return Err(String::from("Max less than or equal to min amount"));
            }
        }

        if let Some(job_interval_ms) = update.job_interval_ms {
            // Cancel old timer if it exists
            if let Some(timer_id) = job.timer_id {
                ic_cdk_timers::clear_timer(timer_id);
            }

            job.job_interval = Duration::from_millis(job_interval_ms);

            // Start new timer with updated configuration
            let timer_id = run_now_then_interval_with_args(job.job_interval, move || {
                crate::jobs::swap_tokens::run(update.exchange_job_id);
            });

            job.timer_id = Some(timer_id);
        }

        Ok(())
    }
}
