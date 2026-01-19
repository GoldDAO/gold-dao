use crate::exchange_job_config::ExchangeJobConfig;
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Args {
    pub exchange_configs: Vec<ExchangeJobConfig>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
}
