use crate::{ApiFeeBucket, VaultError};
use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Deserialize)]
pub struct UpdateVaultArg {
    pub vault_id: u64,
    pub fee_bucket: Option<ApiFeeBucket>,
    pub new_owner: Option<Account>,
}

pub type Args = UpdateVaultArg;
pub type Response = Result<(), VaultError>;

impl fmt::Display for UpdateVaultArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UpdateVaultArg {{ bucket: {:?}, new owner: {:?} }}",
            self.fee_bucket, self.new_owner
        )
    }
}
