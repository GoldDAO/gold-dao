use crate::index::IndexType;
use crate::index::SortBy;
use candid::CandidType;
use icrc_ledger_types::icrc3::blocks::BlockWithId;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize)]
pub struct Args {
    pub start: u64,
    pub length: u64,
    pub filters: Vec<IndexType>,
    pub sort_by: Option<SortBy>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct Response {
    pub total: u64,
    pub blocks: Vec<BlockWithId>,
}
