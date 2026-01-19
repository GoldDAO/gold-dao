use crate::exchange_job_config::ExchangeJobConfig;
use bity_ic_types::BuildVersion;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
    pub authorized_principals: Vec<Principal>,
    pub exchange_configs: Vec<ExchangeJobConfig>,
    pub icp_swap_canister_id: Principal,
}
