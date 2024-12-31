use crate::{types::DisplayAmount, VaultError};
use candid::CandidType;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Deserialize)]
pub struct RepayDebtArg {
    pub debt_amount: u64,
    pub vault_id: u64,
}

pub type Args = RepayDebtArg;
pub type Response = Result<u64, VaultError>;

impl fmt::Display for RepayDebtArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RepayDebtArg {{ margin amount: {} GLDT, vault_id: {} }}",
            DisplayAmount(self.debt_amount),
            self.vault_id
        )
    }
}
