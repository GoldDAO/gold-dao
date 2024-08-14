use serde::{ Deserialize, Serialize };
use candid::Principal;
use super::{ SwapClient, SwapConfig };

#[derive(Serialize, Deserialize, Clone)]
pub struct SwapClinets {
    this_canister_id: Principal,
    pub swap_clients: Vec<Box<dyn SwapClient>>,
}

impl SwapClinets {
    pub fn init(this_canister_id: Principal) -> Self {
        Self { this_canister_id, swap_clients: vec![] }
    }

    pub fn add_swap_client(&mut self, swap_config: SwapConfig) {
        self.swap_clients.push(swap_config.build_swap_client(self.this_canister_id));
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn SwapClient>> {
        self.swap_clients.iter()
    }
}
