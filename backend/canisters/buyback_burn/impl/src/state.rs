use crate::types::token_swaps::TokenSwaps;
use crate::types::token_swaps::TokenSwapsMetrics;
use crate::types::ExchangeJobs;
use bity_ic_canister_state_macros::canister_state;
use bity_ic_types::BuildVersion;
use buyback_burn_api::exchange_job_config::ExchangeJobConfig;
use buyback_burn_api::stake_icp_config::StakeIcpConfig;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use tracing::error;
use types::{Cycles, TimestampMillis};
use utils::env::{CanisterEnv, Environment};
use utils::memory::MemorySize;

canister_state!(RuntimeState);

#[derive(Serialize, Deserialize)]
pub struct RuntimeState {
    pub env: CanisterEnv,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: CanisterEnv, data: Data) -> Self {
        RuntimeState { env, data }
    }

    pub fn is_caller_governance_principal(&self) -> bool {
        self.data.authorized_principals.contains(&self.env.caller())
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            canister_info: CanisterInfo {
                test_mode: self.env.is_test_mode(),
                now: self.env.now(),
                version: self.env.version(),
                commit_hash: self.env.commit_hash().to_string(),
                memory_used: MemorySize::used(),
                cycles_balance: self.env.cycles_balance(),
            },
            authorized_principals: self.data.authorized_principals.to_vec(),
            token_swaps_metrics: self.data.token_swaps.get_metrics(),
            icp_swap_canister_id: self.data.icp_swap_canister_id,
            exchange_jobs: self.data.exchange_jobs.clone(),
            stake_icp_config: self.data.stake_icp_config.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub authorized_principals: Vec<Principal>,
    pub token_swaps: TokenSwaps,

    // NOTE: the main ICP Swap canister
    pub icp_swap_canister_id: Principal,
    pub exchange_jobs: ExchangeJobs,

    // storage for swap guard see guards.rs
    pub exchange_job_guards: BTreeSet<u128>,

    pub stake_icp_config: Option<StakeIcpConfig>,
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        authorized_principals: Vec<Principal>,
        icp_swap_canister_id: Principal,
        exchange_configs: Vec<ExchangeJobConfig>,
        stake_icp_config: Option<StakeIcpConfig>,
    ) -> Self {
        let mut exchange_jobs = ExchangeJobs::init();

        for exchange_job_config in exchange_configs {
            match exchange_job_config.validate() {
                Ok(_) => {
                    let _ = exchange_jobs.add_exchange_job(exchange_job_config);
                }
                Err(e) => {
                    error!("Invalid exchange job config provided during init: {}", e);
                }
            }
        }

        Self {
            authorized_principals: authorized_principals.into_iter().collect(),
            icp_swap_canister_id,
            token_swaps: TokenSwaps::default(),
            exchange_jobs,
            exchange_job_guards: BTreeSet::new(),
            stake_icp_config,
        }
    }
}

#[derive(Serialize)]
pub struct Metrics {
    pub canister_info: CanisterInfo,
    pub authorized_principals: Vec<Principal>,
    pub icp_swap_canister_id: Principal,
    pub token_swaps_metrics: TokenSwapsMetrics,
    pub exchange_jobs: ExchangeJobs,
    pub stake_icp_config: Option<StakeIcpConfig>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterInfo {
    pub now: TimestampMillis,
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub memory_used: MemorySize,
    pub cycles_balance: Cycles,
}
