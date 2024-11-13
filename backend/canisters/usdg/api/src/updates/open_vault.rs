use crate::{ApiFeeBucket, VaultError};
use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct OpenVaultArg {
    pub borrowed_amount: u64,
    pub margin_amount: u64,
    pub fee_bucket: ApiFeeBucket,
    pub maybe_subaccount: Option<[u8; 32]>,
}

#[derive(CandidType, Deserialize)]
pub struct OpenVaultSuccess {
    pub block_index: u64,
    pub vault_id: u64,
}

pub type Args = OpenVaultArg;
pub type Response = Result<OpenVaultSuccess, VaultError>;
