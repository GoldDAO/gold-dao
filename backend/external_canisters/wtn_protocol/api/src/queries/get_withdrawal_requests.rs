use crate::types::Account;
use crate::types::WithdrawalDetails;

pub type Args = Option<Account>;
pub type Response = Vec<WithdrawalDetails>;
