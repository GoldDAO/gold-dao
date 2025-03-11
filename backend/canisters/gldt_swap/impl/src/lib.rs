use ic_cdk::export_candid;
pub mod archive;
mod guards;
mod jobs;
pub mod lifecycle;
mod memory;
pub mod model;
pub mod queries;
pub mod service_status;
pub mod state;
pub mod swap;
pub mod updates;
mod utils;
//test

use archive::*;
use lifecycle::*;
use queries::*;
use updates::*;

export_candid!();
