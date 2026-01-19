use crate::swap_config::ExchangeConfig;
use candid::CandidType;
use ic_ledger_types::Tokens;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::Deserialize;
use serde::Serialize;
use types::TokenSymbol;

#[derive(Serialize, CandidType, Deserialize, Clone, Debug)]
pub struct ExchangeJobConfig {
    pub token_to_sell: TokenSymbol,
    pub token_to_buy: TokenSymbol,
    pub exchange: ExchangeConfig,
    pub rate_per_interval: u64,
    pub job_interval_ms: u64,
    pub source_subaccount: Option<Subaccount>,
    pub min_amount: Tokens,
    pub max_amount: Option<Tokens>,
    pub destination_account: Option<Account>,
}

impl ExchangeJobConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.token_to_sell == self.token_to_buy {
            return Err("Token to sell cannot be the same as token to buy".to_string());
        }

        if self.job_interval_ms < 60_000 {
            return Err("Job interval must be at least 60,000ms".to_string());
        }

        if let Some(max) = self.max_amount {
            if self.min_amount >= max {
                return Err("min_amount must be strictly less than max_amount".to_string());
            }
        }

        if self.rate_per_interval == 0 {
            return Err("rate_per_interval must be greater than 0".to_string());
        }

        Ok(())
    }
}
