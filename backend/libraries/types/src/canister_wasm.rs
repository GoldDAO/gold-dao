use crate::{BuildVersion, CanisterId, Hash};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UpgradeCanisterWasmArgs {
    pub wasm: CanisterWasmCM,
    pub filter: Option<UpgradesFilter>,
    pub use_for_new_canisters: Option<bool>,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct ChunkedCanisterWasm {
    pub wasm: CanisterWasmCM,
    pub chunks: Vec<Hash>,
    pub wasm_hash: Hash,
}

impl From<CanisterWasmCM> for ChunkedCanisterWasm {
    fn from(value: CanisterWasmCM) -> Self {
        ChunkedCanisterWasm {
            wasm: value,
            chunks: Vec::new(),
            wasm_hash: [0; 32],
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct CanisterWasmCM {
    pub version: BuildVersion,
    #[serde(with = "serde_bytes")]
    pub module: Vec<u8>,
}

impl Default for CanisterWasmCM {
    fn default() -> Self {
        CanisterWasmCM {
            version: BuildVersion::new(0, 0, 0),
            module: Vec::default(),
        }
    }
}

impl Debug for CanisterWasmCM {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CanisterWasm")
            .field("version", &self.version)
            .field("byte_length", &self.module.len())
            .finish()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct UpgradesFilter {
    pub include: Vec<CanisterId>,
    pub exclude: Vec<CanisterId>,
}
