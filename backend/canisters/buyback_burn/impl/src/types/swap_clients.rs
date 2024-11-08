use crate::types::*;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
use types::TokenInfo;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct SwapClients {
    pub swap_clients: HashMap<u128, SwapClientEnum>,
}

impl SwapClients {
    pub fn init() -> Self {
        Self {
            swap_clients: HashMap::new(),
        }
    }

    pub fn add_swap_client(
        &mut self,
        input_token: TokenInfo,
        output_token: TokenInfo,
        icp_swap_canister_id: Principal,
    ) {
        let swap_client_id = self.get_next_id();
        let exchange_config = ExchangeConfig::ICPSwap(ICPSwapConfig::new(icp_swap_canister_id));
        self.swap_clients.insert(
            swap_client_id,
            SwapClientEnum::build_swap_client(SwapConfig {
                swap_client_id,
                input_token,
                output_token,
                exchange_config,
            }),
        );
        info!("Added swap client {}", swap_client_id);
    }

    pub fn get_next_id(&self) -> u128 {
        let swap_client_len: u128 = self.swap_clients.len().try_into().unwrap();
        swap_client_len + 1
    }

    pub fn get_swap_client(&self, swap_client_id: u128) -> Option<&SwapClientEnum> {
        self.swap_clients.get(&swap_client_id)
    }

    pub fn iter(&self) -> std::collections::hash_map::Values<'_, u128, SwapClientEnum> {
        self.swap_clients.values()
    }
}
