use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::BuildVersion;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UpgradeArgs {
    pub wasm_version: BuildVersion,
    pub commit_hash: String,
}
