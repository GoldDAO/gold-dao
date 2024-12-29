use icrc_ledger_types::icrc1::account::Account;
use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug, Eq, PartialEq)]
pub struct LiquidationPoolPosition {
    pub gldt_returns: u64,
    pub usdg_available: u64,
}

pub type Args = Option<Account>;
pub type Response = LiquidationPoolPosition;
