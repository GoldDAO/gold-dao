use candid::{ CandidType, Principal };
use serde::Deserialize;

#[derive(Deserialize, CandidType)]
pub struct InitArgs {
    pub test_mode: bool,
    pub authorized_principals: Vec<Principal>,
}
