use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub canister_ids: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Response {}
