use candid::CandidType;
use serde::{ Deserialize, Serialize };
use crate::token_swaps::TokenSwap;

pub type Args = u128;

pub type Response = Option<TokenSwap>;
