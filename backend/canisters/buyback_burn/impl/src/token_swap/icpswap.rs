use super::swap_client::SwapClient;
use async_trait::async_trait;
use ic_cdk::api::call::CallResult;
use icpswap_client::ICPSwapClient;
use icrc_ledger_types::icrc1::account::Account;
use crate::token_swap::SwapConfig;
use crate::token_swap::ExchangeConfig;

#[async_trait]
#[typetag::serde]
// TODO: when async traits would be stable, rewrite without async_trait usage:
// https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html
impl SwapClient for ICPSwapClient {
    fn get_config(&self) -> SwapConfig {
        SwapConfig {
            // FIXME: fix the id
            swap_client_id: 0,
            input_token: self.input_token(),
            output_token: self.output_token(),
            exchange_config: ExchangeConfig::ICPSwap(ICPSwapConfig {
                swap_canister_id: self.swap_canister_id(),
                zero_for_one: self.zero_for_one(),
            }),
        }
    }

    fn clone_box(&self) -> Box<dyn SwapClient> {
        Box::new(self.clone())
    }

    async fn deposit_account(&self) -> CallResult<Account> {
        Ok(self.deposit_account_internal())
    }

    async fn deposit(&self, amount: u128) -> CallResult<()> {
        self.deposit(amount).await.map(|_| ())
    }

    async fn swap(&self, amount: u128, min_amount_out: u128) -> CallResult<Result<u128, String>> {
        self.swap(amount, min_amount_out).await
    }

    async fn withdraw(&self, successful_swap: bool, amount: u128) -> CallResult<u128> {
        self.withdraw(successful_swap, amount).await
    }
}

use types::CanisterId;
use serde::{ Deserialize, Serialize };
use candid::CandidType;
use candid::Principal;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ICPSwapConfig {
    pub swap_canister_id: CanisterId,
    pub zero_for_one: bool,
}

impl Default for ICPSwapConfig {
    fn default() -> Self {
        Self {
            swap_canister_id: Principal::from_text("7eikv-2iaaa-aaaag-qdgwa-cai").unwrap(),
            zero_for_one: true,
        }
    }
}
