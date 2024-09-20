use candid::{ CandidType, Principal };
use serde::Deserialize;

#[derive(Deserialize, CandidType, Debug)]
pub struct InitArgArchive {
    pub test_mode: bool,
    pub authorized_principals: Vec<Principal>,
    pub version: String,
}
