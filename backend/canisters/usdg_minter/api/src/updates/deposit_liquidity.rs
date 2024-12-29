use crate::{types::DisplayAmount, VaultError};
use candid::CandidType;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Deserialize)]
pub struct DepositArg {
    pub deposited_amount: u64,
    pub maybe_subaccount: Option<[u8; 32]>,
}

pub type Args = DepositArg;
pub type Response = Result<u64, VaultError>;

impl fmt::Display for DepositArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DepositArg {{ deposited_amount: {} USDG }}",
            DisplayAmount(self.deposited_amount),
        )
    }
}
