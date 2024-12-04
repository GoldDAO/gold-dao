use candid::CandidType;
use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Args {
    pub burn_rate: Option<u8>,
    pub min_burn_amount: Option<Tokens>,
    pub buyback_interval_in_secs: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    InvalidBurnRate,
}
