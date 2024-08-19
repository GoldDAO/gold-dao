use candid::{Nat, CandidType, Principal};
use serde::Deserialize;

pub struct DSA([u8; 32]);
impl DSA {
  pub fn to_string(&self) -> String {
    return hex::encode(self.0);
  }
}
pub const DEFAULT_SUBACCOUNT: DSA = DSA([0; 32]);


#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetBlocksRequest {
  pub start: Nat,
  pub length: Nat 
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsResponse {
  pub first_index: Nat,
  pub log_length: Nat,
  pub transactions: Vec<Transaction>,
  pub archived_transactions: Vec<ArchivedRange1> // ****
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Transaction  {
  pub burn: Option<Burn>,
  pub kind: String,
  pub mint: Option<Mint>,
  pub approve: Option<Approve>,
  pub timestamp: u64,
  pub transfer: Option<Transfer>
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Account {
  pub owner: Principal,
  pub subaccount: Option<Vec<u8>> 
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Burn  {
pub from: Account,
pub memo: Option<Vec<u8>>,
pub created_at_time: Option<u64>,
pub amount: Nat,
pub spender: Option<Account>,  
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Mint  {
  pub to: Account,
  pub memo: Option<Vec<u8>>,
  pub created_at_time: Option<u64>,
  pub amount: Nat
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Approve {
  pub fee: Option<Nat>,
  pub from: Account,
  pub memo: Option<Vec<u8>>,
  pub created_at_time: Option<u64>,
  pub amount: Nat,
  pub expected_allowance: Option<Nat>,
  pub expires_at: Option<u64>,
  pub spender: Account
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Transfer {
  pub to: Account,
  pub fee: Option<Nat>,
  pub from: Account,
  pub memo: Option<Vec<u8>>,
  pub created_at_time: Option<u64>,
  pub amount: Nat,
  pub spender: Option<Account>
}

candid::define_function!(pub ArchivedRange1Callback : (GetBlocksRequest) -> (
  TransactionRange,
) query);

#[derive(CandidType, Deserialize)]
pub struct ArchivedRange1 {
pub callback: ArchivedRange1Callback,
pub start: candid::Nat,
pub length: candid::Nat,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransactionRange {
  pub transactions: Vec<Transaction>
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetBlocksArgs1 { pub start: Nat, pub length: Nat }

