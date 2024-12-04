use crate::{types::DisplayAmount, VaultError};
use candid::CandidType;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Deserialize)]
pub struct BorrowArg {
    pub borrowed_amount: u64,
    pub vault_id: u64,
}

pub type Args = BorrowArg;
pub type Response = Result<u64, VaultError>;

impl fmt::Display for BorrowArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BorrowArg {{ borrowed_amount: {} USDG, vault_id: {} }}",
            DisplayAmount(self.borrowed_amount),
            self.vault_id
        )
    }
}
