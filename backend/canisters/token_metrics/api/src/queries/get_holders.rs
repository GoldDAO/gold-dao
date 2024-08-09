use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use crate::token_data::{ GetHoldersArgs, WalletOverview };

pub type Args = GetHoldersArgs;
pub type Response = GetHoldersResponse;

#[derive(Serialize, Deserialize, Clone, Default, CandidType)]
pub struct GetHoldersResponse {
  pub data: Vec<(Account, WalletOverview)>,
  pub current_offset: u64,
  pub limit: u64,
  pub total_count: usize,
}