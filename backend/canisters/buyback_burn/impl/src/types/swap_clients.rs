use serde::{ Deserialize, Serialize };
use crate::types::*;
use types::TokenInfo;
use candid::Principal;
use tracing::info;

#[derive(Serialize, Deserialize, Clone)]
pub struct SwapClients {
    pub swap_clients: Vec<SwapClientEnum>,
}

impl SwapClients {
    pub fn init() -> Self {
        Self {
            swap_clients: vec![],
        }
    }

    pub fn add_swap_client(
        &mut self,
        input_token: TokenInfo,
        output_token: TokenInfo,
        icp_swap_canister_id: Principal
    ) {
        let swap_client_id = self.get_next_id();
        let exchange_config = ExchangeConfig::ICPSwap(ICPSwapConfig::new(icp_swap_canister_id));
        self.swap_clients.push(
            SwapClientEnum::build_swap_client(SwapConfig {
                swap_client_id,
                input_token,
                output_token,
                exchange_config,
            })
        );
        info!("Added swap client {}", swap_client_id);
    }

    pub fn get_next_id(&self) -> u128 {
        let swap_client_len: u128 = self.swap_clients.len().try_into().unwrap();
        swap_client_len + 1
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SwapClientEnum> {
        self.swap_clients.iter()
    }

    pub fn into_iter(&self) -> std::vec::IntoIter<swap_client::SwapClientEnum> {
        self.swap_clients.clone().into_iter()
    }
}
