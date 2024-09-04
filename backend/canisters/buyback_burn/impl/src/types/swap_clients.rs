use super::SwapClient;
use serde::{ Deserialize, Serialize };
use crate::utils::build_swap_client;
use crate::types::*;
use types::TokenInfo;
use tracing::{ error, info };

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

    pub fn add_swap_client(
        &mut self,
        swap_client_id: u128,
        input_token: TokenInfo,
        output_token: TokenInfo
    ) -> Result<(), String> {
        let exchange_config = ExchangeConfig::ICPSwap(ICPSwapConfig::new());

        let swap_client = match
            build_swap_client(SwapConfig {
                swap_client_id,
                input_token,
                output_token,
                exchange_config,
            })
        {
            Ok(client) => client,
            Err(e) => {
                error!("Failed to build swap client {}: {}", swap_client_id, e);
                return Err(format!("Failed to build swap client {}", swap_client_id));
            }
        };

        self.swap_clients.push(swap_client);
        info!("Added swap client {}", swap_client_id);
        Ok(())
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn SwapClient>> {
        self.swap_clients.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Box<dyn SwapClient>> {
        self.swap_clients.iter_mut()
    }
}
