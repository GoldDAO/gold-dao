use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc2::transfer_from::TransferFromError;
use serde::Deserialize;

pub mod lifecycle;
pub mod queries;
pub mod updates;

#[derive(CandidType, Deserialize)]
pub enum ApiFeeBucket {
    Low,
    Medium,
    High,
}

#[derive(CandidType, Deserialize)]
pub enum VaultError {
    TransferFromError(TransferFromError),
    AnonymousCaller,
    AmountTooLow { minimum_amount: u64 },
    NoRecentGoldPrice,
}

#[derive(CandidType, Deserialize)]
pub struct ApiVault {
    pub vault_id: u64,
    pub owner: Account,
    pub borrowed_amount: u64,
    pub margin_amount: u64,
    pub fee_bucket: ApiFeeBucket,
}
