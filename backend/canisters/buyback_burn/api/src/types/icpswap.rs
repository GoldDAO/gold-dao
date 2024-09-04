use candid::CandidType;
use candid::Principal;
use serde::{ Deserialize, Serialize };
use types::CanisterId;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ICPSwapConfig {
    pub swap_canister_id: Option<CanisterId>,
    pub zero_for_one: bool,
}

impl ICPSwapConfig {
    pub fn new() -> Self {
        Self {
            swap_canister_id: None,
            zero_for_one: true,
        }
    }

    pub fn set_swap_canister_id(&mut self, swap_canister_id: Principal) {
        self.swap_canister_id = Some(swap_canister_id);
    }
}
