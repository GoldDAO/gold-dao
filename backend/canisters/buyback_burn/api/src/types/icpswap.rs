use candid::CandidType;
use candid::Principal;
use serde::{ Deserialize, Serialize };
use types::CanisterId;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ICPSwapConfig {
    pub swap_canister_id: CanisterId,
    pub zero_for_one: bool,
}

impl ICPSwapConfig {
    pub fn new(swap_canister_id: Principal) -> Self {
        Self {
            swap_canister_id,
            zero_for_one: true,
        }
    }
}
