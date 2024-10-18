use candid::CandidType;
use serde::{ Deserialize, Serialize };
use ic_ledger_types::Tokens;

pub type Args = ();
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    pub burn_rate: u8,
    pub min_burn_amount: Tokens,
}
