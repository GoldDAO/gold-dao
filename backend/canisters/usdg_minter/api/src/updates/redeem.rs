use crate::{types::DisplayAmount, VaultError};
use candid::CandidType;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Deserialize)]
pub struct RedeemArg {
    pub amount: u64,
    pub maybe_subaccount: Option<[u8; 32]>,
}

pub type Args = RedeemArg;
pub type Response = Result<u64, VaultError>;

impl fmt::Display for RedeemArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RedeemArg {{ deposited_amount: {} USDG }}",
            DisplayAmount(self.amount),
        )
    }
}
