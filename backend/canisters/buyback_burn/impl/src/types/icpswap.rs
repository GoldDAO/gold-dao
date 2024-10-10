use super::swap_client::SwapClient;
use async_trait::async_trait;
use ic_cdk::api::call::CallResult;
use icrc_ledger_types::icrc1::account::Account;
use crate::types::*;
use types::CanisterId;

#[async_trait]
// TODO: when async traits would be stable, rewrite without async_trait usage:
// https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html
impl SwapClient for ICPSwapClient {
    fn get_swap_client_id(&self) -> u128 {
        self.client_id()
    }

    fn get_config(&self) -> SwapConfig {
        SwapConfig {
            swap_client_id: self.client_id(),
            input_token: self.input_token(),
            output_token: self.output_token(),
            exchange_config: ExchangeConfig::ICPSwap(ICPSwapConfig {
                swap_canister_id: self.swap_canister_id(),
                zero_for_one: self.zero_for_one(),
            }),
        }
    }

    fn set_swap_canister_id(&mut self, swap_canister_id: CanisterId) {
        self.set_swap_canister_id(swap_canister_id);
    }

    fn clone_box(&self) -> Box<dyn SwapClient> {
        Box::new(self.clone())
    }

    async fn get_quote(
        &self,
        amount: u128,
        min_amount_out: u128
    ) -> CallResult<Result<u128, String>> {
        self.get_quote(amount, min_amount_out).await
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
