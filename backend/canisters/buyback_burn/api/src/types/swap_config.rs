use crate::icpswap::ICPSwapConfig;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::TokenSymbol;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct SwapConfig {
    pub swap_client_id: u128,
    pub input_token: TokenSymbol,
    pub output_token: TokenSymbol,
    pub exchange_config: ExchangeConfig,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum ExchangeConfig {
    ICPSwap(ICPSwapConfig),
}
