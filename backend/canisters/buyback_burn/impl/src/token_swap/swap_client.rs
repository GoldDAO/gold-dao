use async_trait::async_trait;
use ic_cdk::api::call::CallResult;
use icrc_ledger_types::icrc1::account::Account;
use crate::state::SwapConfig;

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait SwapClient {
    fn get_config(&self) -> SwapConfig;
    async fn deposit_account(&self) -> CallResult<Account>;
    async fn deposit(&self, amount: u128) -> CallResult<()>;
    async fn swap(&self, amount: u128, min_amount_out: u128) -> CallResult<Result<u128, String>>;
    async fn withdraw(&self, successful_swap: bool, amount: u128) -> CallResult<u128>;
}
