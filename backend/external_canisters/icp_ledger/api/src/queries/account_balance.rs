use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct Tokens {
    pub e8s: u64,
}

pub type Args = ic_ledger_types::AccountBalanceArgs;
pub type Response = Tokens;
