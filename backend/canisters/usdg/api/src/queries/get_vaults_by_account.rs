use crate::ApiVault;
use icrc_ledger_types::icrc1::account::Account;

pub type Args = Option<Account>;
pub type Response = Vec<ApiVault>;
