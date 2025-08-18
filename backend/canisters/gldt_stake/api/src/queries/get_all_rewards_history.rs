use candid::Nat;
use std::collections::BTreeMap;
use std::collections::HashMap;
use types::TokenSymbol;

pub type Args = ();
pub type Response = BTreeMap<u64, HashMap<TokenSymbol, Nat>>;
