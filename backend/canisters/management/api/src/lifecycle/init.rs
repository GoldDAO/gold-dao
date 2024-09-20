use candid::{ CandidType, Principal };
use serde::{ Deserialize, Serialize };
use types::BuildVersion;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub wasm_version: BuildVersion,
    pub commit_hash: String,
    pub authorized_principals: Vec<Principal>,
}
