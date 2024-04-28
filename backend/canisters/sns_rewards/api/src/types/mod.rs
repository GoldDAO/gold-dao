use std::collections::HashMap;

use candid::{ CandidType, Nat };
use serde::{ Deserialize, Serialize };
use types::{ TokenInfo, TokenSymbol };

pub mod payment_round;

pub type TokenRewardTypes = HashMap<TokenSymbol, TokenInfo>;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Empty {}

pub type ReserveTokenAmounts = HashMap<TokenSymbol, Nat>;
