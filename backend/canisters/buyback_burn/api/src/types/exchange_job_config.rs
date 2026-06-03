use crate::post_transfer_action::PostTransferAction;
use crate::swap_config::ExchangeConfig;
use crate::swap_constraint::SwapConstraint;
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
    pub constraints: Vec<SwapConstraint>,
    pub post_transfer_action: Option<PostTransferAction>,
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

        validate_constraints(&self.constraints)?;

        Ok(())
    }
}

pub fn validate_constraints(constraints: &[SwapConstraint]) -> Result<(), String> {
    for i in 0..constraints.len() {
        for j in (i + 1)..constraints.len() {
            // NOTE: allows to compare only types without its content
            if std::mem::discriminant(&constraints[i]) == std::mem::discriminant(&constraints[j]) {
                return Err(format!("Duplicate constraint type: {:?}", &constraints[i]));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::swap_constraint::SwapConstraint;

    #[test]
    fn no_duplicate_constraints_passes() {
        let result = validate_constraints(&vec![
            SwapConstraint::MinSellRatio(30),
            SwapConstraint::MaxSellRatio(100),
        ]);
        assert!(result.is_ok());
    }

    #[test]
    fn duplicate_min_ratio_fails() {
        let result = validate_constraints(&vec![
            SwapConstraint::MinSellRatio(30),
            SwapConstraint::MinSellRatio(20),
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn duplicate_max_ratio_fails() {
        let result = validate_constraints(&vec![
            SwapConstraint::MaxSellRatio(100),
            SwapConstraint::MaxSellRatio(200),
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn empty_constraints_passes() {
        assert!(validate_constraints(&vec![]).is_ok());
    }
}
