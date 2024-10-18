use ic_cdk::export_candid;
// use gldt_swap_api_canister::types::swap::*;
mod utils;
mod guards;
mod lifecycle;
pub mod model;
pub mod queries;
pub mod state;
mod memory;
pub mod updates;
// use ::types::{ HttpRequest, HttpResponse };

use updates::*;
use queries::*;
use lifecycle::*;

export_candid!();
