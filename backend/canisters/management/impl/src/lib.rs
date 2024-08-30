use ic_cdk::export_candid;
use management_api_canister::init::InitArgs;

mod guards;
pub mod lifecycle;
pub mod memory;
pub mod queries;
pub mod state;
pub mod updates;

use lifecycle::*;
use queries::*;
use updates::*;

export_candid!();
