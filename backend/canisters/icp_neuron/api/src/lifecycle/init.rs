use candid::CandidType;

use serde::{Deserialize, Serialize};

use types::BuildVersion;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub test_mode: bool,
    pub version: BuildVersion,
    pub commit_hash: String,
}
