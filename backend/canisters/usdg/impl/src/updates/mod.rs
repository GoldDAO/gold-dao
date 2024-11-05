use candid::{CandidType, Principal};
use icrc_ledger_types::icrc2::transfer_from::TransferFromError;
use serde::Deserialize;

pub mod timer;
pub mod vault;

#[derive(CandidType, Deserialize)]
pub enum VaultError {
    TransferFromError(TransferFromError),
    AnonymousCaller,
    AmountTooLow { minimum_amount: u64 },
    NoRecentGoldPrice,
}

pub fn reject_anonymous_caller() -> Result<(), VaultError> {
    if ic_cdk::caller() == Principal::anonymous() {
        return Err(VaultError::AnonymousCaller);
    }
    Ok(())
}
