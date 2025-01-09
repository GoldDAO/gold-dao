use std::collections::HashMap;

use candid::Nat;
use gldt_stake_common::reward_tokens::TokenSymbol;

pub type Args = HashMap<TokenSymbol, Nat>;
pub type Response = Result<String, String>;
