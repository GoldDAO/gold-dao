use candid::Nat;
use gldt_swap_common::swap::{SwapIndex, SwapInfo};
use std::collections::HashMap;
use std::collections::HashSet;

pub type Args = HashSet<Nat>;
pub type Response = HashMap<SwapIndex, SwapInfo>;
