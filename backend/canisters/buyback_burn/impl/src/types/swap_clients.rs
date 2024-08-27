use super::{SwapClient, SwapConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SwapClients {
    pub swap_clients: Vec<Box<dyn SwapClient>>,
}

impl SwapClients {
    pub fn init() -> Self {
        Self {
            swap_clients: vec![],
        }
    }

    pub fn add_swap_client(&mut self, swap_config: SwapConfig) {
        self.swap_clients.push(swap_config.build_swap_client());
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn SwapClient>> {
        self.swap_clients.iter()
    }
}
