use candid::Nat;
use std::collections::HashMap;
use types::TokenSymbol;

pub type AllowedRewardTypes = Vec<TokenSymbol>;
pub type RewardTokens = HashMap<TokenSymbol, Nat>;
