use crate::{ApiFeeBucket, ApiVault, VaultError};
use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::Deserialize;

pub type Args = Option<Account>;
pub type Response = Vec<ApiVault>;
