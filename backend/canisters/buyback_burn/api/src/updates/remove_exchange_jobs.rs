use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Args {
    pub exchange_configs: Vec<u128>,
}

impl Args {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
}
