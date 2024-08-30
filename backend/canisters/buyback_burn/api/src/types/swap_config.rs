use candid::CandidType;
use serde::{ Deserialize, Serialize };
use types::TokenInfo;
use crate::icpswap::ICPSwapConfig;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct SwapConfig {
    pub swap_client_id: u128,
    pub input_token: TokenInfo,
    pub output_token: TokenInfo,
    pub exchange_config: ExchangeConfig,
}

impl SwapConfig {
    pub fn new(
        swap_client_id: u128,
        input_token: TokenInfo,
        output_token: TokenInfo,
        exchange_config: ExchangeConfig
    ) -> Self {
        SwapConfig {
            swap_client_id,
            input_token,
            output_token,
            exchange_config,
        }
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum ExchangeConfig {
    ICPSwap(ICPSwapConfig),
}
