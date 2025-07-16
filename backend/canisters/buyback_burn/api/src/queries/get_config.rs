use candid::CandidType;
use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};
use utils::numeric::Percentage;

pub type Args = ();
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    pub burn_percentage: Percentage,
    pub min_burn_amount: Tokens,
}
