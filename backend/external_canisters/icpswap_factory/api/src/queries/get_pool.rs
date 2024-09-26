use candid::CandidType;
use serde::Deserialize;
use crate::ICPSwapResult;
use candid::Principal;

#[derive(CandidType, Deserialize)]
pub struct Args {
    pub fee: candid::Nat,
    pub token0: Token,
    pub token1: Token,
}

pub type Response = ICPSwapResult<PoolData>;

#[derive(CandidType, Deserialize)]
pub struct PoolData {
    pub fee: candid::Nat,
    pub key: String,
    pub tick_spacing: candid::Int,
    pub token0: Token,
    pub token1: Token,
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct Token {
    pub address: String,
    pub standard: String,
}
