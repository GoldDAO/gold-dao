use std::collections::HashMap;
use candid::Nat;
use types::{ TokenInfo, TokenSymbol };

pub mod payment_round;
pub mod subaccounts;

pub type TokenRewardTypes = HashMap<TokenSymbol, TokenInfo>;
pub type ReserveTokenAmounts = HashMap<TokenSymbol, Nat>;
