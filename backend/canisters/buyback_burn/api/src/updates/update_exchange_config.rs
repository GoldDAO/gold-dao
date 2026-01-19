use candid::CandidType;
use ic_ledger_types::Tokens;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Args {
    pub exchange_job_id: u128,
    pub rate_per_interval: Option<u64>,
    pub job_interval_ms: Option<u64>,
    pub source_subaccount: Option<Option<Subaccount>>,
    pub min_amount: Option<Tokens>,
    pub max_amount: Option<Option<Tokens>>,
    pub destination_account: Option<Option<Account>>,
}

impl Args {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(interval) = self.job_interval_ms {
            if interval < 60_000 {
                return Err("Job interval must be at least 60,000ms (1 minute)".to_string());
            }
        }

        if let Some(rate) = self.rate_per_interval {
            if rate == 0 {
                return Err("rate_per_interval must be greater than 0".to_string());
            }
        }

        if let (Some(min), Some(Some(max))) = (self.min_amount, self.max_amount) {
            if min >= max {
                return Err("New min_amount must be less than the new max_amount".to_string());
            }
        }

        Ok(())
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    InvalidBurnRate,
}
