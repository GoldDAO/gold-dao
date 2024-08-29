use candid::CandidType;
use serde::{ Deserialize, Serialize };
use ic_ledger_types::Tokens;
use std::collections::HashMap;
use crate::token_swaps::TokenSwap;

pub type Args = ();

pub type Response = HashMap<u128, TokenSwap>;
