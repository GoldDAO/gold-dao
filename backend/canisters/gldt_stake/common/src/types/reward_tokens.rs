use candid::{Nat, Principal};
use std::collections::HashMap;

pub type RewardTokens = HashMap<String, Nat>;

pub type TokenSymbol = String;
pub type LedgerId = Principal;
pub type LedgerFee = Nat;
pub type RewardTypes = HashMap<TokenSymbol, (LedgerId, LedgerFee)>; // e.g GLDGov -> (ledger_id, fee)
