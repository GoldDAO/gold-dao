use super::SwapClient;
use serde::{ Deserialize, Serialize };
use crate::utils::build_swap_client;
use crate::types::*;
use types::TokenInfo;
use candid::Principal;
use tracing::info;

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
        output_token: TokenInfo,
        icp_swap_canister_id: Principal
        // swap_config: SwapConfig
    ) {
        let exchange_config = ExchangeConfig::ICPSwap(ICPSwapConfig::new(icp_swap_canister_id));
        self.swap_clients.push(
            build_swap_client(SwapConfig {
                swap_client_id,
                input_token,
                output_token,
                exchange_config,
            })
        );
        info!("Added swap client {:?}", swap_client_id);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn SwapClient>> {
        self.swap_clients.iter()
    }
}
