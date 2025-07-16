use candid::Nat;
use std::collections::HashMap;
use types::TokenSymbol;

pub type Args = ();
pub type Response = HashMap<TokenSymbol, Result<Nat, String>>;
