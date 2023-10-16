use candid::{ self, Principal, Nat };
use ic_cdk::api::call::CallResult;
use icrc_ledger_types::icrc1::{
    transfer::{ BlockIndex, TransferArg, TransferError },
    account::Account,
};

type TransferResult = Result<BlockIndex, TransferError>;

pub struct Service(pub Principal);
impl Service {
    pub async fn icrc1_transfer(&self, arg0: TransferArg) -> CallResult<(TransferResult,)> {
        ic_cdk::call(self.0, "icrc1_transfer", (arg0,)).await
    }
    pub async fn icrc1_minting_account(&self) -> CallResult<(Principal,)> {
        ic_cdk::call(self.0, "icrc1_minting_account", ()).await
    }
    pub async fn icrc1_balance_of(&self, account: Account) -> CallResult<(Nat,)> {
        ic_cdk::call(self.0, "icrc1_balance_of", (account,)).await
    }
}
