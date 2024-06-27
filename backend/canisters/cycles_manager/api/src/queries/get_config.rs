use candid::CandidType;
use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};
use types::Cycles;

pub type Args = ();
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    pub max_top_up_amount: Cycles,
    pub min_cycles_balance: Cycles,
    pub icp_burn_amount: Tokens,
}
