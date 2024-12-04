use crate::{types::DisplayAmount, VaultError};
use candid::CandidType;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Deserialize)]
pub struct AddMarginArg {
    pub margin_amount: u64,
    pub vault_id: u64,
}

pub type Args = AddMarginArg;
pub type Response = Result<u64, VaultError>;

impl fmt::Display for AddMarginArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddMarginArg {{ margin amount: {} GLDT, vault_id: {} }}",
            DisplayAmount(self.margin_amount),
            self.vault_id
        )
    }
}
