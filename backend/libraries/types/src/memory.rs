use candid::CandidType;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, CandidType)]
pub struct MemorySize {
    heap: u64,
    stable: u64,
}

impl MemorySize {
    pub fn used() -> Self {
        Self {
            heap: utils::memory::wasm_memory_size(),
            stable: stable_memory::used(),
        }
    }
}
