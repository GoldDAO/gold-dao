use candid::{ CandidType, Principal };
use icpswap_client::ICPSwapClient;
use serde::{ Deserialize, Serialize };
use types::TokenInfo;
use crate::token_swap::icpswap::ICPSwapConfig;
use crate::token_swap::SwapClient;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct SwapConfig {
    pub swap_client_id: u128,
    pub input_token: TokenInfo,
    pub output_token: TokenInfo,
    pub exchange_config: ExchangeConfig,
}

impl Default for SwapConfig {
    fn default() -> Self {
        Self {
            swap_client_id: 0,
            input_token: TokenInfo::icp(),
            output_token: TokenInfo::gldgov(),
            exchange_config: ExchangeConfig::ICPSwap(ICPSwapConfig::default()),
        }
    }
}

impl SwapConfig {
    pub fn build_swap_client(&self, this_canister_id: Principal) -> Box<dyn SwapClient> {
        let input_token = self.input_token.clone();
        let output_token = self.output_token.clone();

        match &self.exchange_config {
            ExchangeConfig::ICPSwap(icpswap) => {
                let (token0, token1) = if icpswap.zero_for_one {
                    (input_token, output_token)
                } else {
                    (output_token, input_token)
                };
                Box::new(
                    ICPSwapClient::new(
                        this_canister_id,
                        icpswap.swap_canister_id,
                        token0,
                        token1,
                        icpswap.zero_for_one
                    )
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum ExchangeConfig {
    ICPSwap(ICPSwapConfig),
}
