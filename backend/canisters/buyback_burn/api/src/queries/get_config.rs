use candid::CandidType;
use serde::{Deserialize, Serialize};

pub type Args = ();
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Response {
    pub min_burn_amount: u128,
}
