use ic_cdk::export_candid;
use lifecycle::init::InitArgs;
use ::types::{ HttpRequest, HttpResponse };

mod jobs;
mod lifecycle;
mod memory;
mod queries;
mod updates;
mod state;
mod types;

export_candid!();
