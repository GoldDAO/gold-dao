use crate::{types::DisplayAmount, ApiFeeBucket, VaultError};
use candid::CandidType;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Deserialize)]
pub struct OpenVaultArg {
    pub borrowed_amount: u64,
    pub margin_amount: u64,
    pub fee_bucket: ApiFeeBucket,
    pub maybe_subaccount: Option<[u8; 32]>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct OpenVaultSuccess {
    pub block_index: u64,
    pub vault_id: u64,
}

pub type Args = OpenVaultArg;
pub type Response = Result<OpenVaultSuccess, VaultError>;

impl fmt::Display for OpenVaultArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OpenVaultArg {{ borrowed_amount: {} USDG, margin_amount: {} GLDT, fee_bucket: {:?}, maybe_subaccount: {} }}",
            DisplayAmount(self.borrowed_amount),
            DisplayAmount(self.margin_amount),
            self.fee_bucket,
            self.maybe_subaccount
                .as_ref()
                .map(|sub| format!("{:?}", sub))
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
