use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::custom_types::IndexerType;


#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TargetArgs {
    pub target_ledger: String,
    pub hourly_size: u8,
    pub daily_size: u8,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InitLedgerArgs {
    pub target: TargetArgs,
    pub index_type: IndexerType, 

}
pub type Args = InitLedgerArgs;
pub type Response = String;