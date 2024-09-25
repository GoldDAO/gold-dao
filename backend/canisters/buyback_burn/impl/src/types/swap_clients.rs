use super::SwapClient;
use serde::{ Deserialize, Serialize };
use crate::utils::build_swap_client;
use crate::types::*;

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
        self.swap_clients.push(build_swap_client(swap_config));
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn SwapClient>> {
        self.swap_clients.iter()
    }
}
