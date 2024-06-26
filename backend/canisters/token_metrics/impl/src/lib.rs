use ic_cdk::export_candid;
use lifecycle::init::InitArgs;
use candid::Principal;
use crate::state::GovernanceStats;
use crate::state::TokenSupplyData;

mod consts;
mod jobs;
mod lifecycle;
mod memory;
mod queries;
mod updates;
mod state;
mod types;

export_candid!();
