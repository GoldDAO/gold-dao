use crate::types::ExchangeConfig;
use crate::types::SwapConfig;
use async_trait::async_trait;
use candid::CandidType;
use enum_dispatch::enum_dispatch;
use ic_cdk::api::call::CallResult;
use icpswap_client::ICPSwapClient;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[async_trait]
#[enum_dispatch(SwapClientEnum)]
pub trait SwapClient {
    fn get_swap_client_id(&self) -> u128;
    fn get_config(&self) -> SwapConfig;
    fn clone_box(&self) -> Box<dyn SwapClient>;
    fn set_swap_canister_id(&mut self, swap_canister_id: CanisterId);
    async fn get_quote(
        &self,
        amount: u128,
        min_amount_out: u128,
    ) -> CallResult<Result<u128, String>>;
    async fn deposit_account(&self) -> CallResult<Account>;
    async fn deposit(&self, amount: u128) -> CallResult<()>;
    async fn swap(&self, amount: u128, min_amount_out: u128) -> CallResult<Result<u128, String>>;
    async fn withdraw(&self, successful_swap: bool, amount: u128) -> CallResult<u128>;
}

impl Clone for Box<dyn SwapClient> {
    fn clone(&self) -> Box<dyn SwapClient> {
        self.clone_box()
    }
}

#[enum_dispatch]
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum SwapClientEnum {
    ICPSwapClient(ICPSwapClient),
}

impl SwapClientEnum {
    pub fn build_swap_client(config: SwapConfig) -> Self {
        let input_token = config.input_token;
        let output_token = config.output_token;

        match config.exchange_config {
            ExchangeConfig::ICPSwap(icpswap) => {
                let (token0, token1) = if icpswap.zero_for_one {
                    (input_token, output_token)
                } else {
                    (output_token, input_token)
                };

                SwapClientEnum::ICPSwapClient(ICPSwapClient::new(
                    config.swap_client_id,
                    ic_cdk::api::id(),
                    icpswap.swap_canister_id,
                    token0,
                    token1,
                    icpswap.zero_for_one,
                ))
            }
        }
    }
}
