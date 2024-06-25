use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::Cycles;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Args {
    pub max_top_up_amount: Option<Cycles>,
    pub min_cycles_balance: Option<Cycles>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
}
