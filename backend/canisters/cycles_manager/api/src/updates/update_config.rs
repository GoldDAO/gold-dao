use candid::CandidType;
use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};
use types::Cycles;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub max_top_up_amount: Option<Cycles>,
    pub min_cycles_balance: Option<Cycles>,
    pub icp_burn_amount: Option<Tokens>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
}
